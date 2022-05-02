use rand::Rng;
use rce::models::solution::*;

const X_API_KEY: (&str, &str) = (
    "x-api-key",
    "dGhpc19pc19leGFtcGxlX3RleHRfZm9yX3NlY3JldF9rZXk=",
);

const EXECUTE_ENDPOINT: &str = "/api/execute";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let solutions = vec![
        SolutionBuilder::make_rust()
            .add_src_from_file("../tests/sum_two_numbers/rust_sol.rs")
            .add_tests_from_file("../tests/sum_two_numbers/input.txt")
            .add_timeout(1000),
        SolutionBuilder::make_python()
            .add_src_from_file("../tests/sum_two_numbers/python_sol.py")
            .add_tests_from_file("../tests/sum_two_numbers/input.txt")
            .add_timeout(1000),
        SolutionBuilder::make_java()
            .add_src_from_file("../tests/sum_two_numbers/java_sol.java")
            .add_tests_from_file("../tests/sum_two_numbers/input.txt")
            .add_timeout(1000),
        SolutionBuilder::make_c()
            .add_src_from_file("../tests/sum_two_numbers/c_sol.c")
            .add_tests_from_file("../tests/sum_two_numbers/input.txt")
            .add_timeout(1000),
        SolutionBuilder::make_cpp()
            .add_src_from_file("../tests/sum_two_numbers/cpp_sol.cpp")
            .add_tests_from_file("../tests/sum_two_numbers/input.txt")
            .add_timeout(1000),
        SolutionBuilder::make_js()
            .add_src_from_file("../tests/sum_two_numbers/js_sol.js")
            .add_tests_from_file("../tests/sum_two_numbers/input.txt")
            .add_timeout(1000),
        SolutionBuilder::make_csharp()
            .add_src_from_file("../tests/sum_two_numbers/csharp_sol.cs")
            .add_tests_from_file("../tests/sum_two_numbers/input.txt")
            .add_timeout(1000),
        SolutionBuilder::make_kotlin()
            .add_src_from_file("../tests/sum_two_numbers/kotlin_sol.kt")
            .add_tests_from_file("../tests/sum_two_numbers/input.txt")
            .add_timeout(1000),
        SolutionBuilder::make_pascal()
            .add_src_from_file("../tests/sum_two_numbers/pascal_sol.pas")
            .add_tests_from_file("../tests/sum_two_numbers/input.txt")
            .add_timeout(1000),
    ];

    println!("Input number of requests: ");
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).unwrap();
    let number = number.trim().parse::<u64>().unwrap();

    let reqs = {
        let mut reqs = vec![];

        for i in 0..=number {
            let sol = solutions[rand::thread_rng().gen_range(0..solutions.len())]
                .clone()
                .build_with_uuid(&i.to_string());
            reqs.push(
                reqwest::Client::new()
                    .get("http://localhost:8000".to_owned() + EXECUTE_ENDPOINT)
                    .header(X_API_KEY.0, X_API_KEY.1)
                    .json(&sol)
                    .send(),
            );
        }

        reqs
    };

    let time = std::time::Instant::now();
    let _reqs = futures::future::join_all(reqs).await;

    println!("Duration: {:?}", time.elapsed());

    Ok(())
}
