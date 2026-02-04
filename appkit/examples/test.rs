use std::time::Instant;

mod test_cases;

const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

pub fn prompt_yes_no(question: &str, no_message: &str) -> Result<(), String> {
    use std::io::{self, Write};

    print!("{} (y/n): ", question);
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("failed to read input: {}", e))?;

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => Ok(()),
        "n" | "no" => Err(no_message.to_string()),
        _ => Err("invalid response".to_string()),
    }
}

pub fn expected_error(result: Result<(), String>, expected: &str) -> Result<(), String> {
    match result {
        Ok(()) => Err("expected error, got Ok".to_string()),
        Err(err) if err.contains(expected) => Ok(()),
        Err(err) => Err(format!("unexpected error: {}", err)),
    }
}

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
