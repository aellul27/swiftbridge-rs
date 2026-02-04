use std::thread;
use appkit::app::App;

use super::{SubTestResult, TestCase};

pub fn test_case() -> TestCase {
    TestCase {
        name: "app_tests",
        func: run_subtests,
    }
}

fn run_subtests() -> Vec<SubTestResult> {
    let mut results = Vec::new();

    // test if the "must be called on the main thread" is working
    let expected_error = "swift_appkit_create_app must be called on the main thread";

    let handle = thread::spawn(|| App::create().map(|_| ()).map_err(|e| e.to_string()));

    let result = match handle.join() {
        Ok(result) => crate::expected_error(result, expected_error),
        Err(_) => Err("app thread panicked".to_string()),
    };

    results.push(SubTestResult {
        name: "app_create on non main thread",
        result,
    });

    // test if the app can be created
    let app_result = match App::create() {
        Ok(_app) => crate::prompt_yes_no(
            "Is there now an executable in your dock that is not responding?",
            "app not launched",
        ),
        Err(e) => Err(format!("Failed to create app: {}", e)),
    };

    results.push(SubTestResult {
        name: "app_create on main thread",
        result: app_result,
    });


    results
}