use crate::lib::tokenizer::Token;

struct DemoNode {}

pub enum Node {
    DemoNode(DemoNode),
}

pub fn parse(tokens: &[Token]) -> Vec<Node> {
    vec![Node::DemoNode(DemoNode {})]
}
