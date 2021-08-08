use crate::Arithmetic;
use crate::Arithmetic::*;
use crate::Operation::*;

pub struct Token {
    kind: Arithmetic,
    values: Vec<Box<Token>>,
}

impl Token {
    pub fn new(kind: Arithmetic) -> Self {
        Self {
            kind,
            values: Vec::new(),
        }
    }

    pub fn to_string(&self) -> String {
        match self.kind {
            Num(val) => val.to_string(),
            Op(Add) => self
                .values
                .iter()
                .map(|token| token.to_string())
                .collect::<Vec<_>>()
                .join(" + "),
            Op(Sub) => self
                .values
                .iter()
                .enumerate()
                .map(|(i, token)| match token.kind {
                    Op(Sub) if i != 0 => format!("({})", token.to_string()),
                    _ => token.to_string(),
                })
                .collect::<Vec<_>>()
                .join(" - "),
            Op(Mul) => self
                .values
                .iter()
                .map(|token| match token.kind {
                    Op(Add) | Op(Sub) => format!("({})", token.to_string()),
                    _ => token.to_string(),
                })
                .collect::<Vec<_>>()
                .join(" x "),
            Op(Div) => self
                .values
                .iter()
                .enumerate()
                .map(|(i, token)| match token.kind {
                    Op(Add) | Op(Sub) => format!("({})", token.to_string()),
                    Op(Mul) | Op(Div) if i != 0 => format!("({})", token.to_string()),
                    _ => token.to_string(),
                })
                .collect::<Vec<_>>()
                .join(" / "),
        }
    }
}

pub struct TokenTree {
    root: Token,
}

impl TokenTree {
    pub fn new(rpn: &[Arithmetic; 7]) -> Self {
        let mut stack = Vec::with_capacity(3);
        for a in rpn.iter() {
            let mut token = Token::new(*a);
            match a {
                Num(_) => {
                    stack.push(token);
                }
                Op(_) => {
                    let token2 = stack.pop().unwrap();
                    let token1 = stack.pop().unwrap();
                    token.values.push(Box::new(token1));
                    token.values.push(Box::new(token2));
                    stack.push(token);
                }
            }
        }

        Self {
            root: stack.pop().unwrap(),
        }
    }

    pub fn to_string(&self) -> String {
        self.root.to_string()
    }
}

pub fn tokenize(rpn: &[Arithmetic; 7]) -> TokenTree {
    TokenTree::new(rpn)
}
