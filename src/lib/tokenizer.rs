struct DemoToken {}

pub enum Token {
    DemoToken(DemoToken),
}

pub fn tokenize(vyn_code: &str) -> Vec<Token> {
    vec![Token::DemoToken(DemoToken {})]
}
