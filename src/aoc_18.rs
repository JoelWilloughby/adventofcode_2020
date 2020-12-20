#[derive(Debug, Clone)]
pub enum Expression {
    Constant(usize),
    Add(Vec<Expression>),
    Mul(Vec<Expression>),
    AddMul(Vec<(Expression, char)>),
}

fn trim_u8(mut t: &[u8]) -> &[u8] {
    while t.len() > 0 && (t[0] as char).is_whitespace() {
        t = &t[1..]
    }

    t
}

impl Expression {
    fn num_from_str(mut input: &[u8]) -> (Self, &[u8]) {
        let mut acc = 0;
        while input.len() > 0 && (input[0] as char).is_digit(10) {
            acc *= 10;
            acc += (input[0] - '0' as u8) as usize;
            input = &input[1..];
        }

        (Self::Constant(acc), trim_u8(input))
    }

    fn parse_operand(mut rem: &[u8]) -> (Self, &[u8]) {
        rem = trim_u8(rem);
        let c = rem[0] as char;
        if c == '(' {
            let (lhs, mut rem) = Self::parse_bytes(&rem[1..]).unwrap();
            // look for closing paren
            rem = trim_u8(rem);
            assert_eq!(rem[0], ')' as u8);
            (lhs, trim_u8(&rem[1..]))
        } else {
            // Know you have at least 1 lhs
            Self::num_from_str(rem)
        }
    }

    fn parse_bytes(mut rem: &[u8]) -> Option<(Self, &[u8])> {
        rem = trim_u8(rem);
        let (lhs, mut rem) = Self::parse_operand(rem);

        if rem.len() == 0 {
            return Some((lhs, rem));
        }

        match rem[0] as char{
            '+' | '*' => (),
            _ => {
                // This guy is done
                return Some((lhs, rem));
            }
        }

        let mut args = vec![(lhs, '+')];
        while rem.len() > 0 && (rem[0] == '+' as u8 || rem[0] == '*' as u8) {
            let (operand, inner_rem) = Self::parse_operand(&rem[1..]);
            args.push((operand, rem[0] as char));
            rem = trim_u8(inner_rem);
        }

        Some((Self::AddMul(args), trim_u8(rem)))
    }

    pub fn from_str(input: &str) -> Option<Self> {
        if let Some(exp) = Self::parse_bytes(&input.as_bytes()) {
            Some(exp.0)
        } else {
            None
        }
    }

    pub fn set_precedences(&self) -> Self {
        match self {
            Self::Constant(val) => Self::Constant(*val),
            Self::AddMul(this_args) => {
                let (lhs, _) = &this_args[0];
                let mut add_args = vec![lhs.set_precedences()];
                let mut mult_args = vec![];
                for (arg, op) in this_args.iter().skip(1) {
                    if *op == '*' {
                        if add_args.len() > 1 {
                            mult_args.push(Self::Add(add_args.clone()));
                        } else if add_args.len() > 0 {
                            mult_args.push(add_args.pop().unwrap());
                        }
                        add_args.clear();
                    }
                    add_args.push(arg.set_precedences());
                }

                if add_args.len() > 1 {
                    mult_args.push(Self::Add(add_args));
                } else if add_args.len() > 0 {
                    mult_args.push(add_args.pop().unwrap());
                }

                if mult_args.len() > 1 {
                    Self::Mul(mult_args)
                } else {
                    mult_args.pop().unwrap()
                }
            },
            Self::Add(args) => Self::Add(args.to_vec()),
            Self::Mul(args) => Self::Mul(args.to_vec()),
        }
    }

    pub fn eval(&self) -> usize {
        match self {
            Self::AddMul(args) => {
                let mut acc = 0;
                for (arg, op) in args.iter() {
                    let val = arg.eval();
                    match op {
                        '+' => {acc += val},
                        '*' => {acc *= val},
                        _ => {panic!("Bad operator");}
                    }
                }
                acc
            },
            Self::Constant(val) => {
                *val
            },
            Self::Mul(args) => {
                args.iter().fold(1, |acc, arg| acc * arg.eval())
            },
            Self::Add(args) => {
                args.iter().fold(0, |acc, arg| acc + arg.eval())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drive(filename: &str) {
        let input = std::fs::read_to_string(filename).unwrap();
        let sum = input.lines().fold(0, |acc, line|
            acc + Expression::from_str(line).unwrap().eval()
        );
        println!("Part 1: {}", sum);
        let sum = input.lines().fold(0, |acc, line|
            acc + Expression::from_str(line).unwrap().set_precedences().eval()
        );
        println!("Part 2: {}", sum);
    }

    #[test]
    fn it_works() {
        let test_strs: [(&str, usize, usize); 7] = [
            ("(3)", 3, 3),
            ("1 + 2 + 3", 6, 6),
            ("(1 + 2 + 3 * (4 + 6)) + (3 + 5)", 68, 68),
            ("2 * 3 + (4 * 5)", 26, 46),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240, 669060),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437, 1445),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632, 23340)
        ];
        for (test, test_val, test_val_2) in &test_strs {
            let exp = Expression::from_str(test).unwrap();
            assert_eq!(exp.eval(), *test_val);
            let exp = exp.set_precedences();
            assert_eq!(exp.eval(), *test_val_2);
        }
    }

    #[test]
    fn test_it() {
        drive("res/18/input.txt");
    }
}