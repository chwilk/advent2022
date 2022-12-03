use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;
use std::fs;

// Borrowed from command line rust
type TestResult = Result<(), Box<dyn Error>>;

struct TestDay {
    day: String,
    args: Vec<String>,
    half: String,
    bin: Option<String>,
}

// Day 1
#[test]
fn day1a() -> TestResult {
    TestDay::day("1").half("a").test_file().assert()
}
#[test]
fn day1b() -> TestResult {
    TestDay::day("1").half("b").test_file().assert()
}
#[test]
fn day1() -> TestResult {
    TestDay::day("1").half("").test_file().assert()
}

// Day 2
#[test]
fn day2() -> TestResult {
    TestDay::day("2").half("").test_file().assert()
}

// Day 3
#[test]
fn day3() -> TestResult {
    TestDay::day("3").half("").test_file().assert()
}

// Day 21 (example with arguments)
/*
#[test]
fn day21() -> TestResult {
    TestDay::day("21")
        .half("")
        .arg("4")
        .arg("8")
        .assert()
}
*/

impl TestDay {
    pub fn day(day: &str) -> TestDay {
        TestDay {
            day: day.to_string(),
            args: Vec::new(),
            half: String::from(""),
            bin: None,
        }
    }

    // add argument
    #[allow(dead_code)]
    pub fn arg<'a>(&'a mut self, arg: &str) -> &'a mut TestDay {
        self.args.push(arg.to_string());
        self
    }

    // add test filename (optional)
    pub fn test_file<'a>(&'a mut self) -> &'a mut TestDay {
        self.args.push(format!("tests/inputs/test{}.dat", self.day));
        self
    }

    // add a,b half of day designator (optional)
    pub fn half<'a>(&'a mut self, half: &str) -> &'a mut TestDay {
        self.half.push_str(half);
        self
    }

    // set binary (default advent{day}{half}) (optional)
    #[allow(dead_code)]
    pub fn set_bin<'a>(&'a mut self, bin: &str) -> &'a mut TestDay {
        self.bin = Some(bin.to_string());
        self
    }

    // Do the test run
    pub fn assert<'a>(&'a mut self) -> TestResult {
        let expected = fs::read_to_string(format!("tests/outputs/test{}{}.out", self.day, self.half))?;

        let bin = match &self.bin {
            None    => format!("day{}{}", self.day, self.half),
            Some(b) => b.to_string(),
        };

        Command::cargo_bin(bin)?
            .args(&self.args)
            .assert()
            .stdout(predicate::str::contains(expected.trim_end()));

        Ok(())
    }
}
