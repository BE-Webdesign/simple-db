use std::iter::Peekable;

#[cfg(test)]
mod tests {
    use crate::LexItem;

    use crate::get_number;
    use crate::lex;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn get_number_works() {
        let mut iter = "99".chars().peekable();

        if let Some(&c) = iter.peek() {
            iter.next();
            let num = get_number(c, &mut iter);

            assert_eq!(num, 99);
        }
    }

    #[test]
    fn lex_works() {
        let input = "(99 + 1) * 4".to_string();

        let tokens: Result<Vec<LexItem>, String> = lex(&input);

        let mut tokens_iter = tokens.unwrap().into_iter();

        assert_eq!(Some(LexItem::Paren('(')), tokens_iter.next());
        assert_eq!(Some(LexItem::Num(99)), tokens_iter.next());
        assert_eq!(Some(LexItem::Op('+')), tokens_iter.next());
        assert_eq!(Some(LexItem::Num(1)), tokens_iter.next());
        assert_eq!(Some(LexItem::Paren(')')), tokens_iter.next());
        assert_eq!(Some(LexItem::Op('*')), tokens_iter.next());
        assert_eq!(Some(LexItem::Num(4)), tokens_iter.next());
        assert_eq!(None, tokens_iter.next());
    }
}

#[derive(Debug, Clone)]
pub enum GrammarItem {
    Product,
    Sum,
    Number(u64),
    Paren,
}

#[derive(Debug, Clone)]
pub struct ParseNode {
    pub children: Vec<ParseNode>,
    pub entry: GrammarItem,
}

impl ParseNode {
    pub fn new() -> ParseNode {
        ParseNode {
            children: Vec::new(),
            entry: GrammarItem::Paren,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexItem {
    Paren(char),
    Op(char),
    Num(u64),
}

fn lex(input: &String) -> Result<Vec<LexItem>, String> {
    let mut result = Vec::new();

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                it.next();
                let n = get_number(c, &mut it);
                result.push(LexItem::Num(n));
            }
            '+' | '*' => {
                result.push(LexItem::Op(c));
                it.next();
            }
            '(' | ')' | '[' | ']' | '{' | '}' => {
                result.push(LexItem::Paren(c));
                it.next();
            }
            ' ' => {
                it.next();
            }
            _ => {
                return Err(format!("unexpected character {}", c));
            }
        }
    }
    Ok(result)
}

fn get_number<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> u64 {
    let mut number = c
        .to_string()
        .parse::<u64>()
        .expect("The caller should have passed a digit.");
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<u64>()) {
        number = number * 10 + digit;
        iter.next();
    }
    number
}

pub fn parse(input: &String) -> Result<ParseNode, String> {
    let tokens = lex(input)?;
    parse_expr(&tokens, 0).and_then(|(n, i)| {
        if i == tokens.len() {
            Ok(n)
        } else {
            Err(format!(
                "Expected end of input, found {:?} at {}",
                tokens[i], i
            ))
        }
    })
}

fn parse_expr(tokens: &Vec<LexItem>, pos: usize) -> Result<(ParseNode, usize), String> {
    let (node_summand, next_pos) = parse_summand(tokens, pos)?;
    let c = tokens.get(next_pos);
    match c {
        Some(&LexItem::Op('+')) => {
            // recurse on the expr
            let mut sum = ParseNode::new();
            sum.entry = GrammarItem::Sum;
            sum.children.push(node_summand);
            let (rhs, i) = parse_expr(tokens, next_pos + 1)?;
            sum.children.push(rhs);
            Ok((sum, i))
        }
        _ => {
            // we have just the summand production, nothing more.
            Ok((node_summand, next_pos))
        }
    }
}

pub const GREETING: &'static str = "Hallo, Rust library here!\n";
