// appkit/test_cases/mod.rs

mod testing_the_tester;
mod app;

pub struct SubTestResult {
    pub name: &'static str,
    pub result: Result<(), String>,
}

pub struct TestCase {
    pub name: &'static str,
    pub subtests: i32,
    pub func: fn() -> Vec<SubTestResult>,
}

pub fn all_tests() -> Vec<TestCase> {
    vec![
        testing_the_tester::test_case(),
        app::test_case(),
    ]
}
