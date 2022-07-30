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
    Paren
}

pub const GREETING: &'static str = "Hallo, Rust library here!\n";