use std::i64;

use crate::equation;

#[derive(Debug)]
pub struct Equation {
    pub expect: i64,
    pub values: Vec<i64>,
}

impl From::<&str> for Equation {
    fn from(value: &str) -> Self {
        let (left, right) = value.split_once(':').unwrap(); // TODO
        let result: i64 = left
            .trim()
            .parse()
            .unwrap(); // TODO

        let values: Vec<_> = right
            .trim()
            .split_whitespace()
            .map(|token| token.parse::<i64>().unwrap()) // TODO
            .collect();

        Self { expect: result, values }
    }
}

impl Equation {
    pub fn is_possible(&self) -> bool {
        fn num_digits(number: i64) -> i64 {
            let mut number = number;
            let mut p = 0;

            while number != 0 {
                number /= 10;
                p += 1;
            }

            p
        }

        fn concat(lhs: i64, rhs: i64) -> i64 {
            lhs * 10_i64.pow(num_digits(rhs) as u32) + rhs
        }

        fn backtrack(total: i64, equation: &Equation) -> bool {
            if equation.values.len() > 0 {
                let mut values = equation.values.clone();
                let top = values.remove(0);

                let equation = Equation { expect: equation.expect, values };

                backtrack(total + top, &equation) ||
                backtrack(total * top, &equation) ||
                backtrack(concat(total, top), &equation)
            } else {
                assert_eq!(equation.values, vec![]);
                equation.expect == total
            }
        }

        backtrack(0, self)
    }
}
