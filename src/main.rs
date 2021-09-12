use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

mod token;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Arithmetic {
    Num(i32),
    Op(Operation),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Operation {
    Add,
    Sub,
    Div,
    Mul,
}

use Arithmetic::*;
use Operation::*;

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "x",
            Self::Div => "/",
        };
        write!(f, "{}", op)
    }
}

const INDEXES: [[usize; 4]; 24] = [
    [0, 1, 2, 3],
    [0, 1, 3, 2],
    [0, 2, 1, 3],
    [0, 2, 3, 1],
    [0, 3, 1, 2],
    [0, 3, 2, 1],
    [1, 0, 2, 3],
    [1, 0, 3, 2],
    [1, 2, 0, 3],
    [1, 2, 3, 0],
    [1, 3, 0, 2],
    [1, 3, 2, 0],
    [2, 0, 1, 3],
    [2, 0, 3, 1],
    [2, 1, 0, 3],
    [2, 1, 3, 0],
    [2, 3, 0, 1],
    [2, 3, 1, 0],
    [3, 0, 1, 2],
    [3, 0, 2, 1],
    [3, 1, 0, 2],
    [3, 1, 2, 0],
    [3, 2, 0, 1],
    [3, 2, 1, 0],
];

const OPS: [(Arithmetic, Arithmetic, Arithmetic); 64] = [
    (Op(Add), Op(Add), Op(Add)),
    (Op(Add), Op(Add), Op(Sub)),
    (Op(Add), Op(Add), Op(Mul)),
    (Op(Add), Op(Add), Op(Div)),
    (Op(Add), Op(Sub), Op(Add)),
    (Op(Add), Op(Sub), Op(Sub)),
    (Op(Add), Op(Sub), Op(Mul)),
    (Op(Add), Op(Sub), Op(Div)),
    (Op(Add), Op(Mul), Op(Add)),
    (Op(Add), Op(Mul), Op(Sub)),
    (Op(Add), Op(Mul), Op(Mul)),
    (Op(Add), Op(Mul), Op(Div)),
    (Op(Add), Op(Div), Op(Add)),
    (Op(Add), Op(Div), Op(Sub)),
    (Op(Add), Op(Div), Op(Mul)),
    (Op(Add), Op(Div), Op(Div)),
    (Op(Sub), Op(Add), Op(Add)),
    (Op(Sub), Op(Add), Op(Sub)),
    (Op(Sub), Op(Add), Op(Mul)),
    (Op(Sub), Op(Add), Op(Div)),
    (Op(Sub), Op(Sub), Op(Add)),
    (Op(Sub), Op(Sub), Op(Sub)),
    (Op(Sub), Op(Sub), Op(Mul)),
    (Op(Sub), Op(Sub), Op(Div)),
    (Op(Sub), Op(Mul), Op(Add)),
    (Op(Sub), Op(Mul), Op(Sub)),
    (Op(Sub), Op(Mul), Op(Mul)),
    (Op(Sub), Op(Mul), Op(Div)),
    (Op(Sub), Op(Div), Op(Add)),
    (Op(Sub), Op(Div), Op(Sub)),
    (Op(Sub), Op(Div), Op(Mul)),
    (Op(Sub), Op(Div), Op(Div)),
    (Op(Mul), Op(Add), Op(Add)),
    (Op(Mul), Op(Add), Op(Sub)),
    (Op(Mul), Op(Add), Op(Mul)),
    (Op(Mul), Op(Add), Op(Div)),
    (Op(Mul), Op(Sub), Op(Add)),
    (Op(Mul), Op(Sub), Op(Sub)),
    (Op(Mul), Op(Sub), Op(Mul)),
    (Op(Mul), Op(Sub), Op(Div)),
    (Op(Mul), Op(Mul), Op(Add)),
    (Op(Mul), Op(Mul), Op(Sub)),
    (Op(Mul), Op(Mul), Op(Mul)),
    (Op(Mul), Op(Mul), Op(Div)),
    (Op(Mul), Op(Div), Op(Add)),
    (Op(Mul), Op(Div), Op(Sub)),
    (Op(Mul), Op(Div), Op(Mul)),
    (Op(Mul), Op(Div), Op(Div)),
    (Op(Div), Op(Add), Op(Add)),
    (Op(Div), Op(Add), Op(Sub)),
    (Op(Div), Op(Add), Op(Mul)),
    (Op(Div), Op(Add), Op(Div)),
    (Op(Div), Op(Sub), Op(Add)),
    (Op(Div), Op(Sub), Op(Sub)),
    (Op(Div), Op(Sub), Op(Mul)),
    (Op(Div), Op(Sub), Op(Div)),
    (Op(Div), Op(Mul), Op(Add)),
    (Op(Div), Op(Mul), Op(Sub)),
    (Op(Div), Op(Mul), Op(Mul)),
    (Op(Div), Op(Mul), Op(Div)),
    (Op(Div), Op(Div), Op(Add)),
    (Op(Div), Op(Div), Op(Sub)),
    (Op(Div), Op(Div), Op(Mul)),
    (Op(Div), Op(Div), Op(Div)),
];

fn main() {
    let mut numbers = [0; 4];
    let mut rpns = Vec::new();
    let mut old_numbers = HashSet::new();
    let mut results = HashSet::new();
    std::env::args()
        .skip(1)
        .enumerate()
        .for_each(|(id, num)| numbers[id] = num.parse().unwrap());

    let mut nums = [Num(0); 4];
    for index in INDEXES.iter() {
        index
            .iter()
            .enumerate()
            .for_each(|(i, idx)| nums[i] = Num(numbers[*idx]));

        // filter out repeated `nums`
        // If input has duplicated numbers e.g. 2 7 7 13
        // index [0, 1, 2, 3] and [0, 2, 1, 3] will produce same `nums`
        if old_numbers.contains(&nums) {
            continue;
        } else {
            old_numbers.insert(nums);
        }

        for (op1, op2, op3) in OPS.iter().copied() {
            let calculation = [
                [nums[0], nums[1], op1, nums[2], op2, nums[3], op3],
                [nums[0], nums[1], op1, nums[2], nums[3], op2, op3],
                [nums[0], nums[1], nums[2], op1, op2, nums[3], op3],
                [nums[0], nums[1], nums[2], op1, nums[3], op2, op3],
                [nums[0], nums[1], nums[2], nums[3], op1, op2, op3],
            ];

            for cal in &calculation {
                if calculate(cal) == 24. {
                    rpns.push(*cal);
                    break;
                }
            }
        }
    }

    if rpns.len() == 0 {
        println!("Cannot get 24 from {:?}", numbers);
        return;
    }

    let mut id = 1;
    for rpn in rpns.iter() {
        let string = token::tokenize(rpn).to_string();
        if !results.contains(&string) {
            println!("{:<02}. {}", id, string);
            results.insert(string);
            id += 1;
        }
    }
}

fn calculate(rpn: &[Arithmetic; 7]) -> f32 {
    let mut stack = Vec::with_capacity(3);
    for c in rpn.iter() {
        match c {
            Num(val) => {
                stack.push(*val as f32);
            }
            Op(op) => {
                let val2 = stack.pop().unwrap();
                let mut val1 = stack.pop().unwrap();
                match op {
                    Add => val1 += val2,
                    Sub => val1 -= val2,
                    Mul => val1 *= val2,
                    Div => val1 /= val2,
                }
                stack.push(val1);
            }
        }
    }
    stack.pop().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate1() {
        let calculation = [
            Num(1),
            Num(2),
            Op(Operation::Add),
            Num(1),
            Num(7),
            Op(Operation::Add),
            Op(Operation::Mul),
        ];
        assert_eq!(calculate(&calculation), 24.)
    }

    #[test]
    fn test_calculate2() {
        let calculation = [
            Num(13),
            Num(7),
            Num(7),
            Op(Operation::Div),
            Op(Operation::Sub),
            Num(2),
            Op(Operation::Mul),
        ];
        assert_eq!(calculate(&calculation), 24.);
    }
}
