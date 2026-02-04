use std::thread;
use appkit::app::App;

use super::{SubTestResult, TestCase};

pub fn test_case() -> TestCase {
    TestCase {
        name: "app_tests",
        subtests: 2,
        func: run_subtests,
    }
}

fn run_subtests() -> Vec<SubTestResult> {
    let mut results = Vec::new();

    // app create: non-main thread
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

    // app create: main thread
    let (app_result, app) = match App::create() {
        Ok(app) => (
            crate::prompt_yes_no(
                "Is there now an executable in your dock that is not responding?",
                "app not launched",
            ),
            Some(app),
        ),
        Err(e) => (Err(format!("Failed to create app: {}", e)), None),
    };

    results.push(SubTestResult {
        name: "app_create on main thread",
        result: app_result.clone(),
    });

    // app run test: non-main thread
    let app = if let Some(app) = app {
        app
    } else {
        return results;
    };

    results
}