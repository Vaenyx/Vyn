use crate::lib::parser::Node;

pub struct DemoDesugaredNode {}

pub enum DesugaredNode {
    DemoDesugaredNode(DemoDesugaredNode),
}

pub fn desugar(nodes: &[Node]) -> Vec<DesugaredNode> {
    vec![DesugaredNode::DemoDesugaredNode(DemoDesugaredNode {})]
}
