#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
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

#[derive(Debug, Clone)]
pub enum LexItem {
    Paren(char),
    Op(char),
    Num(u64),
}

pub const GREETING: &'static str = "Hallo, Rust library here!\n";
