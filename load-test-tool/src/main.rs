mod request_info;
mod test_info;

use iced::{
    button, futures, pick_list, slider, text_input, Application, Button, Checkbox, Column, Command,
    Container, Element, Length, PickList, Row, Settings, Slider, Text, TextInput,
};
use rand::Rng;
use rce::models::executed_test::*;
use rce::models::solution::*;
use std::collections::HashMap;

use request_info::RequestInfo;
use test_info::TestInfo;

const X_API_KEY: (&str, &str) = (
    "x-api-key",
    "dGhpc19pc19leGFtcGxlX3RleHRfZm9yX3NlY3JldF9rZXk=",
);

const EXECUTE_ENDPOINT: &str = "/api/execute";

lazy_static::lazy_static! {
    static ref EXECUTE_FULL_ENDPOINT: String = {
        let port = std::env::var("RUST_SERVICE_PORT").unwrap_or_else(|_| "8000".to_string()).parse().unwrap_or(8000);
        format!("http://localhost:{}{}", port, EXECUTE_ENDPOINT)
    };

    static ref RAW_SOLUTIONS: HashMap<String, SolutionBuilder> = {
        let mut solutions = HashMap::new();
        solutions.insert("c".to_string(), SolutionBuilder::make_c());
        solutions.insert("cpp".to_string(), SolutionBuilder::make_cpp());
        solutions.insert("csharp".to_string(), SolutionBuilder::make_csharp());
        solutions.insert("java".to_string(), SolutionBuilder::make_java());
        solutions.insert("kotlin".to_string(), SolutionBuilder::make_kotlin());
        solutions.insert("rust".to_string(), SolutionBuilder::make_rust());
        solutions.insert("js".to_string(), SolutionBuilder::make_js());
        solutions.insert("python".to_string(), SolutionBuilder::make_python());
        solutions.insert("pascal".to_string(), SolutionBuilder::make_pascal());

        solutions
    };

    static ref LANGS_EXT: HashMap<String, &'static str> = {
        let mut exts = HashMap::new();
        exts.insert("c".to_string(), "c");
        exts.insert("cpp".to_string(), "cpp");
        exts.insert("csharp".to_string(), "cs");
        exts.insert("java".to_string(), "java");
        exts.insert("kotlin".to_string(), "kt");
        exts.insert("rust".to_string(), "rs");
        exts.insert("js".to_string(), "js");
        exts.insert("python".to_string(), "py");
        exts.insert("pascal".to_string(), "pas");

        exts
    };
}

fn main() -> iced::Result {
    let _ = dotenv::dotenv();
    LoadTestTool::run(Settings::default())
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LoadTestMessage {
    ToggleC(bool),
    ToggleCpp(bool),
    ToggleCsharp(bool),
    ToggleJava(bool),
    ToggleKotlin(bool),
    ToggleRust(bool),
    ToggleJs(bool),
    TogglePython(bool),
    TogglePascal(bool),

    TimeoutChanged(u32),
    DelayChanged(u32),

    TestSelected(String),

    ReqCountChanged(String),

    SendRequest,
    GetResponse(TestInfo),
}

struct LoadTestTool {
    langs: HashMap<String, bool>,

    timeout_slider: slider::State,
    timeout: u32,

    delay_slider: slider::State,
    delay: u32,

    tests_list: pick_list::State<String>,
    tests: Vec<String>,
    test: String,

    reqs_count_text_input: text_input::State,
    reqs_count: u32,

    send_button: Option<button::State>,

    result: Option<TestInfo>,
}

impl LoadTestTool {
    async fn send_request(req_info: RequestInfo) -> TestInfo {
        let mut test_info = TestInfo::new(req_info.count);
        let create_time = std::time::Instant::now();

        let reqs = {
            let mut reqs = vec![];

            let langs = req_info
                .langs
                .iter()
                .filter(|l| *l.1)
                .map(|l| l.0)
                .collect::<Vec<_>>();

            for i in 0..req_info.count {
                let random_lang = langs[rand::thread_rng().gen_range(0..langs.len())];
                test_info.add_lang_req(random_lang);

                let sol = RAW_SOLUTIONS[random_lang].clone();
                let sol = sol
                    .add_src_from_file(format!(
                        "tests/{}/{}_sol.{}",
                        req_info.test_name, random_lang, LANGS_EXT[random_lang]
                    ))
                    .add_timeout(req_info.timeout as u64)
                    .add_tests_from_file(format!("tests/{}/input.txt", req_info.test_name))
                    .build_with_uuid(&i.to_string());

                let delay = rand::thread_rng().gen_range(0..=(req_info.delay as u64));
                test_info.add_generated_delay(delay);

                reqs.push(tokio::spawn({
                    async_std::task::sleep(std::time::Duration::from_millis(delay)).await;
                    reqwest::Client::new()
                        .get(EXECUTE_FULL_ENDPOINT.clone())
                        .header(X_API_KEY.0, X_API_KEY.1)
                        .json(&sol)
                        .send()
                }));
            }

            reqs
        };

        test_info.add_create_time(create_time.elapsed());

        let test_time = std::time::Instant::now();
        let resp = futures::future::join_all(reqs).await;

        test_info.add_test_time(test_time.elapsed());

        resp.into_iter().for_each(|r| {
            if let Ok(Ok(r)) = r {
                let r = futures::executor::block_on(async { r.json::<ExecutedResponse>().await });

                if let Ok(r) = r {
                    test_info.add_execute_time(r.get_avg_execute_time());
                    test_info.add_memory(r.get_avg_memory());
                }
            }
        });

        test_info
    }

    fn toggle_lang(&mut self, lang: &str, mode: bool) {
        if !mode {
            let active_lang_coung = self.langs.iter().filter(|l| *l.1).count();
            if active_lang_coung > 1 {
                self.langs.insert(lang.to_string(), mode);
            }
        } else {
            self.langs.insert(lang.to_string(), mode);
        }
    }
}

impl Application for LoadTestTool {
    type Executor = iced::executor::Default;

    type Message = LoadTestMessage;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let all_tests: Vec<String> = {
            let all_test_folders = std::fs::read_dir("tests").unwrap();
            all_test_folders
                .into_iter()
                .map(|f| f.unwrap())
                .filter(|f| f.path().is_dir())
                .map(|f| f.file_name())
                .map(|f| f.to_string_lossy().to_string())
                .collect()
        };

        let first_test = all_tests[0].clone();

        (
            LoadTestTool {
                langs: {
                    let mut langs = HashMap::new();
                    langs.insert("c".to_string(), true);
                    langs.insert("cpp".to_string(), true);
                    langs.insert("csharp".to_string(), true);
                    langs.insert("java".to_string(), true);
                    langs.insert("kotlin".to_string(), true);
                    langs.insert("rust".to_string(), true);
                    langs.insert("js".to_string(), true);
                    langs.insert("python".to_string(), true);
                    langs.insert("pascal".to_string(), true);

                    langs
                },

                timeout_slider: Default::default(),
                timeout: 1000,

                delay_slider: Default::default(),
                delay: 0,

                tests_list: Default::default(),
                tests: all_tests,
                test: first_test,

                reqs_count_text_input: Default::default(),
                reqs_count: 1,

                send_button: Some(Default::default()),

                result: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Load test tool for Rust Code Executor Service".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            LoadTestMessage::ToggleC(val) => self.toggle_lang("c", val),
            LoadTestMessage::ToggleCpp(val) => self.toggle_lang("cpp", val),
            LoadTestMessage::ToggleCsharp(val) => self.toggle_lang("csharp", val),
            LoadTestMessage::ToggleJava(val) => self.toggle_lang("java", val),
            LoadTestMessage::ToggleKotlin(val) => self.toggle_lang("kotlin", val),
            LoadTestMessage::ToggleRust(val) => self.toggle_lang("rust", val),
            LoadTestMessage::ToggleJs(val) => self.toggle_lang("js", val),
            LoadTestMessage::TogglePython(val) => self.toggle_lang("python", val),
            LoadTestMessage::TogglePascal(val) => self.toggle_lang("pascal", val),
            _ => {}
        };

        match message {
            LoadTestMessage::TimeoutChanged(val) => self.timeout = val,
            LoadTestMessage::DelayChanged(val) => self.delay = val,
            _ => {}
        };

        if let LoadTestMessage::TestSelected(ref val) = message {
            self.test = val.clone()
        }

        if let LoadTestMessage::ReqCountChanged(ref val) = message {
            let reqs = val.parse::<u32>();
            if let Ok(reqs) = reqs {
                self.reqs_count = reqs;
            }
        }

        match message {
            LoadTestMessage::SendRequest => {
                self.send_button = None;
                return Command::perform(
                    Self::send_request(RequestInfo::new(
                        self.langs.clone(),
                        self.reqs_count,
                        self.test.clone(),
                        self.delay,
                        self.timeout,
                    )),
                    LoadTestMessage::GetResponse,
                );
            }
            LoadTestMessage::GetResponse(test_info) => {
                self.send_button = Some(Default::default());
                self.result = Some(test_info);
            }
            _ => {}
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let title = Column::new()
            .width(Length::Shrink)
            .push(Text::new("Load Test Tool").size(40));

        let languages = Column::new()
            .spacing(10)
            .push(Checkbox::new(
                self.langs["c"],
                "C",
                LoadTestMessage::ToggleC,
            ))
            .push(Checkbox::new(
                self.langs["cpp"],
                "C++",
                LoadTestMessage::ToggleCpp,
            ))
            .push(Checkbox::new(
                self.langs["csharp"],
                "C#",
                LoadTestMessage::ToggleCsharp,
            ))
            .push(Checkbox::new(
                self.langs["java"],
                "Java",
                LoadTestMessage::ToggleJava,
            ))
            .push(Checkbox::new(
                self.langs["kotlin"],
                "Kotlin",
                LoadTestMessage::ToggleKotlin,
            ))
            .push(Checkbox::new(
                self.langs["rust"],
                "Rust",
                LoadTestMessage::ToggleRust,
            ))
            .push(Checkbox::new(
                self.langs["js"],
                "JavaScript",
                LoadTestMessage::ToggleJs,
            ))
            .push(Checkbox::new(
                self.langs["python"],
                "Python",
                LoadTestMessage::TogglePython,
            ))
            .push(Checkbox::new(
                self.langs["pascal"],
                "Pascal",
                LoadTestMessage::TogglePascal,
            ));

        let timeout_slider = Slider::new(
            &mut self.timeout_slider,
            1000..=20000,
            self.timeout,
            LoadTestMessage::TimeoutChanged,
        )
        .step(500);

        let delay_slider = Slider::new(
            &mut self.delay_slider,
            0..=1000,
            self.delay,
            LoadTestMessage::DelayChanged,
        )
        .step(50);

        let pick_test = PickList::new(
            &mut self.tests_list,
            &self.tests[..],
            Some(self.test.clone()),
            LoadTestMessage::TestSelected,
        )
        .padding(4);

        let reqs_count = TextInput::new(
            &mut self.reqs_count_text_input,
            "",
            &self.reqs_count.to_string(),
            LoadTestMessage::ReqCountChanged,
        )
        .padding(4);

        let mut left_content = Column::new()
            .spacing(10)
            .push(title)
            .push(
                Column::new()
                    .push(Text::new("Programming languages:"))
                    .push(languages)
                    .spacing(5),
            )
            .push(
                Column::new()
                    .push(Text::new("Timeout:"))
                    .push(
                        Row::new()
                            .push(Text::new("1000").width(Length::Fill))
                            .push(Text::new("20000").width(Length::Shrink)),
                    )
                    .push(timeout_slider)
                    .push(
                        Text::new(format!("{} ms", self.timeout))
                            .width(Length::Fill)
                            .horizontal_alignment(iced::alignment::Horizontal::Center),
                    )
                    .spacing(5),
            )
            .push(
                Column::new()
                    .push(Text::new("Sending delay:"))
                    .push(
                        Row::new()
                            .push(Text::new("0").width(Length::Fill))
                            .push(Text::new("1000").width(Length::Shrink)),
                    )
                    .push(delay_slider)
                    .push(
                        Text::new(format!("{} ms", self.delay))
                            .width(Length::Fill)
                            .horizontal_alignment(iced::alignment::Horizontal::Center),
                    )
                    .spacing(5),
            )
            .push(
                Column::new()
                    .push(Text::new("Select test folder:"))
                    .push(pick_test)
                    .spacing(5),
            )
            .push(
                Column::new()
                    .push(Text::new("Input request count:"))
                    .push(reqs_count)
                    .spacing(5),
            )
            .padding(20)
            .max_width(400);

        if let Some(ref mut button) = self.send_button {
            let button = Button::new(button, Text::new("Execute"))
                .on_press(LoadTestMessage::SendRequest)
                .padding(4);
            left_content = left_content.push(button);
        }

        let mut content = Row::new().push(left_content);

        if let Some(ref result) = self.result {
            content = content.push(result.get_info());
        }

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
