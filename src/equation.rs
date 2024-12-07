use std::i64;

use crate::{concat::Concat, equation};

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
    pub fn is_possible(
        &self,
        operations: Vec<fn(i64, i64) -> i64>
    ) -> bool {
        fn backtrack(
            total: i64, 
            equation: &Equation,
            operations: &Vec<fn(i64, i64) -> i64>
        ) -> bool {
            if equation.values.len() > 0 {
                let mut values = equation.values.clone();
                let top = values.remove(0);

                let equation = Equation { expect: equation.expect, values };

                let mut chain = false;
                for op in operations {
                    chain = chain || backtrack(op(total, top), &equation, operations);
                }

                chain
            } else {
                assert_eq!(equation.values, vec![]);
                equation.expect == total
            }
        }

        backtrack(0, self, &operations)
    }
}
