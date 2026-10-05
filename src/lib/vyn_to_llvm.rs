use crate::lib::{
    desugarer::desugar, generator::generate, parser::parse, tokenizer::tokenize,
    validator::validate,
};

pub fn vyn_to_llvm(input: String) -> String {
    let tokens = tokenize(&input);
    let nodes = parse(&tokens);
    let desugared_nodes = desugar(&nodes);

    validate(&desugared_nodes);

    generate(&desugared_nodes)
}
