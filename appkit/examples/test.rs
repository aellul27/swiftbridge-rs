use std::time::Instant;

mod test_cases;

const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

fn main() {
    println!("running {} tests", test_cases::all_tests().len());

    let mut passed = 0;
    let mut failed = 0;

    let suite_start = Instant::now();

    for test in test_cases::all_tests() {
        let mut subtests_failed: Vec<(&'static str, String)> = Vec::new();
        let mut subtests_passed = 0;

        let subtests = (test.func)();
        for subtest in &subtests {
            match &subtest.result {
                Ok(_) => {
                    subtests_passed += 1;
                }
                Err(err) => {
                    subtests_failed.push((subtest.name, err.clone()));
                }
            }
        }

        let total_subtests = subtests.len();

        if subtests_failed.is_empty() {
            passed += 1;
            println!(
                "{}: [{}/{}]",
                test.name,
                subtests_passed,
                total_subtests
            );
        } else {
            failed += 1;
            println!(
                "{}: [{}/{}]",
                test.name,
                subtests_passed,
                total_subtests
            );
            for (name, err) in subtests_failed {
                println!("    {}: {}", name, err);
            }
        }

    }

    let total_time = suite_start.elapsed();

    println!();
    if failed == 0 {
        println!(
            "{}test result: OK{} {} passed; {} failed; finished in {:.2?}",
            GREEN, RESET, passed, failed, total_time
        );
    } else {
        println!(
            "{}test result: FAILED{} {} passed; {} failed; finished in {:.2?}",
            RED, RESET, passed, failed, total_time
        );
    }

    if failed > 0 {
        std::process::exit(1);
    }
}
