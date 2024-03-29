#[cfg(test)]
mod tests {
    use crate::{execute, SecretKey};
    use actix_web::web::Data;
    use actix_web::{test, web, App};
    use rust_code_executor_service::models::executed_test::{ExecuteStatus, ExecutedResponse};
    use rust_code_executor_service::models::solution::SolutionBuilder;

    const X_API_KEY: (&str, &str) = (
        "x-api-key",
        "dGhpc19pc19leGFtcGxlX3RleHRfZm9yX3NlY3JldF9rZXk=",
    );

    const EXECUTE_ENDPOINT: &str = "/api/execute";

    #[actix_web::test]
    #[should_panic]
    async fn test_without_x_api_key() {
        dotenv::dotenv().ok();
        let pool = Data::new(
            rayon::ThreadPoolBuilder::new()
                .num_threads(4)
                .build()
                .unwrap(),
        );

        let app = test::init_service(
            App::new().service(
                web::scope("/api")
                    .wrap(SecretKey)
                    .app_data(pool)
                    .service(execute),
            ),
        )
        .await;

        let solution = SolutionBuilder::make_rust().build();

        let req = test::TestRequest::get()
            .uri(EXECUTE_ENDPOINT)
            .set_json(solution)
            .to_request();

        test::call_service(&app, req).await;
    }

    #[actix_web::test]
    #[should_panic]
    async fn test_with_wrong_x_api_key() {
        dotenv::dotenv().ok();
        let pool = Data::new(
            rayon::ThreadPoolBuilder::new()
                .num_threads(4)
                .build()
                .unwrap(),
        );

        let app = test::init_service(
            App::new().service(
                web::scope("/api")
                    .wrap(SecretKey)
                    .app_data(pool)
                    .service(execute),
            ),
        )
        .await;

        let solution = SolutionBuilder::make_rust().build();

        let req = test::TestRequest::get()
            .uri(EXECUTE_ENDPOINT)
            .append_header(("x-api-key", "Wrong Key"))
            .set_json(solution)
            .to_request();

        test::call_service(&app, req).await;
    }

    #[actix_web::test]
    async fn test_rust_sum_two_numbers() {
        dotenv::dotenv().ok();
        let pool = Data::new(
            rayon::ThreadPoolBuilder::new()
                .num_threads(4)
                .build()
                .unwrap(),
        );

        let app = test::init_service(
            App::new().service(
                web::scope("/api")
                    .wrap(SecretKey)
                    .app_data(pool)
                    .service(execute),
            ),
        )
        .await;

        let solution = SolutionBuilder::make_rust()
            .add_src_from_file("tests/sum_two_numbers/rust_sol.rs")
            .add_tests_from_file("tests/sum_two_numbers/input.txt")
            .add_timeout(1000)
            .build();

        let req = test::TestRequest::get()
            .uri(EXECUTE_ENDPOINT)
            .append_header(X_API_KEY)
            .set_json(solution)
            .to_request();
        let resp: ExecutedResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(*resp.get_status(), ExecuteStatus::OK);
        let answers = resp
            .get_raw_answers()
            .iter()
            .map(|test| test.trim())
            .collect::<Vec<_>>();
        assert_eq!(answers[0], "2");
        assert_eq!(answers[1], "11");
        assert_eq!(answers[2], "200");
        assert_eq!(answers[3], "1024");
        assert_eq!(answers[4], "2222222211");
    }

    #[actix_web::test]
    async fn test_python_sum_two_numbers() {
        dotenv::dotenv().ok();
        let pool = Data::new(
            rayon::ThreadPoolBuilder::new()
                .num_threads(4)
                .build()
                .unwrap(),
        );

        let app = test::init_service(
            App::new().service(
                web::scope("/api")
                    .wrap(SecretKey)
                    .app_data(pool)
                    .service(execute),
            ),
        )
        .await;

        let solution = SolutionBuilder::make_python()
            .add_src_from_file("tests/sum_two_numbers/python_sol.py")
            .add_tests_from_file("tests/sum_two_numbers/input.txt")
            .add_timeout(1000)
            .build();

        let req = test::TestRequest::get()
            .uri(EXECUTE_ENDPOINT)
            .append_header(X_API_KEY)
            .set_json(solution)
            .to_request();

        let resp: ExecutedResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(*resp.get_status(), ExecuteStatus::OK);
        let answers = resp
            .get_raw_answers()
            .iter()
            .map(|test| test.trim())
            .collect::<Vec<_>>();
        assert_eq!(answers[0], "2");
        assert_eq!(answers[1], "11");
        assert_eq!(answers[2], "200");
        assert_eq!(answers[3], "1024");
        assert_eq!(answers[4], "2222222211");
    }

    #[actix_web::test]
    async fn test_java_sum_two_numbers() {
        dotenv::dotenv().ok();
        let pool = Data::new(
            rayon::ThreadPoolBuilder::new()
                .num_threads(4)
                .build()
                .unwrap(),
        );

        let app = test::init_service(
            App::new().service(
                web::scope("/api")
                    .wrap(SecretKey)
                    .app_data(pool)
                    .service(execute),
            ),
        )
        .await;

        let solution = SolutionBuilder::make_java()
            .add_src_from_file("tests/sum_two_numbers/java_sol.java")
            .add_tests_from_file("tests/sum_two_numbers/input.txt")
            .add_timeout(1000)
            .build();

        let req = test::TestRequest::get()
            .uri(EXECUTE_ENDPOINT)
            .append_header(X_API_KEY)
            .set_json(solution)
            .to_request();

        let resp: ExecutedResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(*resp.get_status(), ExecuteStatus::OK);
        let answers = resp
            .get_raw_answers()
            .iter()
            .map(|test| test.trim())
            .collect::<Vec<_>>();
        assert_eq!(answers[0], "2");
        assert_eq!(answers[1], "11");
        assert_eq!(answers[2], "200");
        assert_eq!(answers[3], "1024");
        assert_eq!(answers[4], "2222222211");
    }

    #[actix_web::test]
    async fn test_c_sum_two_numbers() {
        dotenv::dotenv().ok();
        let pool = Data::new(
            rayon::ThreadPoolBuilder::new()
                .num_threads(4)
                .build()
                .unwrap(),
        );

        let app = test::init_service(
            App::new().service(
                web::scope("/api")
                    .wrap(SecretKey)
                    .app_data(pool)
                    .service(execute),
            ),
        )
        .await;

        let solution = SolutionBuilder::make_c()
            .add_src_from_file("tests/sum_two_numbers/c_sol.c")
            .add_tests_from_file("tests/sum_two_numbers/input.txt")
            .add_timeout(1000)
            .build();

        let req = test::TestRequest::get()
            .uri(EXECUTE_ENDPOINT)
            .append_header(X_API_KEY)
            .set_json(solution)
            .to_request();

        let resp: ExecutedResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(*resp.get_status(), ExecuteStatus::OK);
        let answers = resp
            .get_raw_answers()
            .iter()
            .map(|test| test.trim())
            .collect::<Vec<_>>();
        assert_eq!(answers[0], "2");
        assert_eq!(answers[1], "11");
        assert_eq!(answers[2], "200");
        assert_eq!(answers[3], "1024");
        assert_eq!(answers[4], "2222222211");
    }

    #[actix_web::test]
    async fn test_cpp_sum_two_numbers() {
        dotenv::dotenv().ok();
        let pool = Data::new(
            rayon::ThreadPoolBuilder::new()
                .num_threads(4)
                .build()
                .unwrap(),
        );

        let app = test::init_service(
            App::new().service(
                web::scope("/api")
                    .wrap(SecretKey)
                    .app_data(pool)
                    .service(execute),
            ),
        )
        .await;

        let solution = SolutionBuilder::make_cpp()
            .add_src_from_file("tests/sum_two_numbers/cpp_sol.cpp")
            .add_tests_from_file("tests/sum_two_numbers/input.txt")
            .add_timeout(1000)
            .build();

        let req = test::TestRequest::get()
            .uri(EXECUTE_ENDPOINT)
            .append_header(X_API_KEY)
            .set_json(solution)
            .to_request();

        let resp: ExecutedResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(*resp.get_status(), ExecuteStatus::OK);
        let answers = resp
            .get_raw_answers()
            .iter()
            .map(|test| test.trim())
            .collect::<Vec<_>>();
        assert_eq!(answers[0], "2");
        assert_eq!(answers[1], "11");
        assert_eq!(answers[2], "200");
        assert_eq!(answers[3], "1024");
        assert_eq!(answers[4], "2222222211");
    }

    #[actix_web::test]
    async fn test_js_sum_two_numbers() {
        dotenv::dotenv().ok();
        let pool = Data::new(
            rayon::ThreadPoolBuilder::new()
                .num_threads(4)
                .build()
                .unwrap(),
        );

        let app = test::init_service(
            App::new().service(
                web::scope("/api")
                    .wrap(SecretKey)
                    .app_data(pool)
                    .service(execute),
            ),
        )
        .await;

        let solution = SolutionBuilder::make_js()
            .add_src_from_file("tests/sum_two_numbers/js_sol.js")
            .add_tests_from_file("tests/sum_two_numbers/input.txt")
            .add_timeout(1000)
            .build();

        let req = test::TestRequest::get()
            .uri(EXECUTE_ENDPOINT)
            .append_header(X_API_KEY)
            .set_json(solution)
            .to_request();

        let resp: ExecutedResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(*resp.get_status(), ExecuteStatus::OK);
        let answers = resp
            .get_raw_answers()
            .iter()
            .map(|test| test.trim())
            .collect::<Vec<_>>();
        assert_eq!(answers[0], "2");
        assert_eq!(answers[1], "11");
        assert_eq!(answers[2], "200");
        assert_eq!(answers[3], "1024");
        assert_eq!(answers[4], "2222222211");
    }

    #[actix_web::test]
    async fn test_csharp_sum_two_numbers() {
        dotenv::dotenv().ok();
        let pool = Data::new(
            rayon::ThreadPoolBuilder::new()
                .num_threads(4)
                .build()
                .unwrap(),
        );

        let app = test::init_service(
            App::new().service(
                web::scope("/api")
                    .wrap(SecretKey)
                    .app_data(pool)
                    .service(execute),
            ),
        )
        .await;

        let solution = SolutionBuilder::make_csharp()
            .add_src_from_file("tests/sum_two_numbers/csharp_sol.cs")
            .add_tests_from_file("tests/sum_two_numbers/input.txt")
            .add_timeout(1000)
            .build();

        let req = test::TestRequest::get()
            .uri(EXECUTE_ENDPOINT)
            .append_header(X_API_KEY)
            .set_json(solution)
            .to_request();

        let resp: ExecutedResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(*resp.get_status(), ExecuteStatus::OK);
        let answers = resp
            .get_raw_answers()
            .iter()
            .map(|test| test.trim())
            .collect::<Vec<_>>();
        assert_eq!(answers[0], "2");
        assert_eq!(answers[1], "11");
        assert_eq!(answers[2], "200");
        assert_eq!(answers[3], "1024");
        assert_eq!(answers[4], "2222222211");
    }

    #[actix_web::test]
    async fn test_kotlin_sum_two_numbers() {
        dotenv::dotenv().ok();
        let pool = Data::new(
            rayon::ThreadPoolBuilder::new()
                .num_threads(4)
                .build()
                .unwrap(),
        );

        let app = test::init_service(
            App::new().service(
                web::scope("/api")
                    .wrap(SecretKey)
                    .app_data(pool)
                    .service(execute),
            ),
        )
        .await;

        let solution = SolutionBuilder::make_kotlin()
            .add_src_from_file("tests/sum_two_numbers/kotlin_sol.kt")
            .add_tests_from_file("tests/sum_two_numbers/input.txt")
            .add_timeout(1000)
            .build();

        let req = test::TestRequest::get()
            .uri(EXECUTE_ENDPOINT)
            .append_header(X_API_KEY)
            .set_json(solution)
            .to_request();

        let resp: ExecutedResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(*resp.get_status(), ExecuteStatus::OK);
        let answers = resp
            .get_raw_answers()
            .iter()
            .map(|test| test.trim())
            .collect::<Vec<_>>();
        assert_eq!(answers[0], "2");
        assert_eq!(answers[1], "11");
        assert_eq!(answers[2], "200");
        assert_eq!(answers[3], "1024");
        assert_eq!(answers[4], "2222222211");
    }

    #[actix_web::test]
    async fn test_pascal_sum_two_numbers() {
        dotenv::dotenv().ok();
        let pool = Data::new(
            rayon::ThreadPoolBuilder::new()
                .num_threads(4)
                .build()
                .unwrap(),
        );

        let app = test::init_service(
            App::new().service(
                web::scope("/api")
                    .wrap(SecretKey)
                    .app_data(pool)
                    .service(execute),
            ),
        )
        .await;

        let solution = SolutionBuilder::make_pascal()
            .add_src_from_file("tests/sum_two_numbers/pascal_sol.pas")
            .add_tests_from_file("tests/sum_two_numbers/input.txt")
            .add_timeout(1000)
            .build();

        let req = test::TestRequest::get()
            .uri(EXECUTE_ENDPOINT)
            .append_header(X_API_KEY)
            .set_json(solution)
            .to_request();

        let resp: ExecutedResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(*resp.get_status(), ExecuteStatus::OK);
        let answers = resp
            .get_raw_answers()
            .iter()
            .map(|test| test.trim())
            .collect::<Vec<_>>();
        assert_eq!(answers[0], "2");
        assert_eq!(answers[1], "11");
        assert_eq!(answers[2], "200");
        assert_eq!(answers[3], "1024");

        //В паскале нет 64-битных интов
        assert_ne!(answers[4], "2222222211");
    }
}
