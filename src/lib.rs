#![feature(conservative_impl_trait)]

use std::fmt;

#[macro_export]
macro_rules! assert_that {
    ( $actual: expr, panics ) => {
        let result = std::panic::catch_unwind(|| { $actual; });
        if result.is_ok() {
            panic!("\nFailed assertion; expected expression to panic")
        }
    };
    ( $actual: expr, does not panic ) => {
        let result = std::panic::catch_unwind(|| { $actual; });
        if result.is_err() {
            panic!("\nFailed assertion; expected expression to panic")
        }
    };
    ( $actual: expr, $matcher: expr ) => {
        match $matcher.check(&$actual) {
            MatchResult::Matched { .. } => { },
            MatchResult::Failed { name, reason } => {
                panic!("\nFailed assertion of matcher: {}\n{}", name, reason)
            }
        }
    };
}


pub trait Matcher<T> {
    fn check(&mut self, actual: &T) -> MatchResult;
}

impl<T, F> Matcher<T> for F
where F: FnMut(&T) -> MatchResult {
    fn check(&mut self, actual: &T) -> MatchResult {
        self(actual)
    }
}

pub enum MatchResult {
    Matched {
        name: String
    },
    Failed {
        name: String,
        reason: String
    }
}


pub fn format_fail_reason(reason: &str) -> String {
    format!("  Because: {}", reason)
}

pub fn format_fail_comparison<T>(actual: T, expected: T) -> String
where T: fmt::Debug {
    format!("  Expected: {:?}\n  Got: {:?}", expected, actual)
}

pub mod matchers;
