use super::{SubTestResult, TestCase};

pub fn test_case() -> TestCase {
    TestCase {
        name: "testing_the_tester",
        func: run_subtests,
    }
}

fn run_subtests() -> Vec<SubTestResult> {
    let mut results = Vec::new();

    let mut value = 42;
    results.push(SubTestResult {
        name: "create_int",
        result: Ok(()),
    });

    results.push(SubTestResult {
        name: "is_number",
        result: Ok(()),
    });

    value += 8;
    results.push(SubTestResult {
        name: "add_to_int",
        result: Ok(()),
    });

    let expected = 50;
    results.push(SubTestResult {
        name: "check_result",
        result: if value == expected {
            Ok(())
        } else {
            Err(format!("expected {}, got {}", expected, value))
        },
    });

    results
}
