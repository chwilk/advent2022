#!/bin/bash

TOMORROW=$(date -d tomorrow +'%d')
DAY=${1:-${TOMORROW}}

if [ ! -f src/bin/day${DAY}.rs ] 
then
    cp src/bin/day{1,${DAY}}.rs
    touch tests/inputs/test${DAY}.dat
    touch tests/outputs/test${DAY}.out
    cat <<END >> tests/examples.rs

// Day ${DAY}
#[test]
fn day${DAY}() -> TestResult {
    TestDay::day("${DAY}").half("").test_file().assert()
}
END
fi