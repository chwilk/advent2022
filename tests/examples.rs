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

// Day 4
#[test]
fn day4() -> TestResult {
    TestDay::day("4").half("").test_file().assert()
}

// Day 5
#[test]
fn day5() -> TestResult {
    TestDay::day("5").half("").test_file().assert()
}

// Day 6
#[test]
fn day6() -> TestResult {
    TestDay::day("6").half("").test_file().assert()
}

// Day 7
#[test]
fn day7() -> TestResult {
    TestDay::day("7").half("").test_file().assert()
}

// Day 8
#[test]
fn day8() -> TestResult {
    TestDay::day("8").half("").test_file().assert()
}

// Day 9
#[test]
fn day9() -> TestResult {
    TestDay::day("9").half("").test_file().assert()
}

// Day 10
#[test]
fn day10() -> TestResult {
    TestDay::day("10").half("").test_file().assert()
}

// Day 11
#[test]
fn day11() -> TestResult {
    TestDay::day("11").half("").test_file().assert()
}
// Day 12
#[test]
fn day12() -> TestResult {
    TestDay::day("12").half("").test_file().assert()
}

// Day 17
#[test]
fn day17() -> TestResult {
    TestDay::day("17").half("").test_file().assert()
}

// Day 19
//#[test]
//fn day19() -> TestResult {
//    TestDay::day("19").half("").test_file().assert()
//}

// Day 21
#[test]
fn day21() -> TestResult {
    TestDay::day("21").half("").test_file().assert()
}

// Day 20
#[test]
fn day20() -> TestResult {
    TestDay::day("20").half("").test_file().assert()
}
