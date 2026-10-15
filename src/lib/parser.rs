use crate::lib::tokenizer::Token;

pub struct DemoNode {}

pub enum Node {
    DemoNode(DemoNode),
}

pub fn parse(_tokens: &[Token]) -> Vec<Node> {
    vec![Node::DemoNode(DemoNode {})]
}
