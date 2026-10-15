use crate::parsing::Node;

pub struct DemoDesugaredNode {}

pub enum DesugaredNode {
    DemoDesugaredNode(DemoDesugaredNode),
}

pub fn desugar(_nodes: &[Node]) -> Vec<DesugaredNode> {
    vec![DesugaredNode::DemoDesugaredNode(DemoDesugaredNode {})]
}
