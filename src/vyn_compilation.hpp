#pragma once

#include "generation.hpp"
#include "parser.hpp"
#include "tokenization.hpp"

#include <algorithm>
#include <cstdlib>
#include <iostream>
#include <string>

std::string vyn_to_c(const std::string &input) {
  Tokenizer tokenizer(input);
  std::vector<Token> tokens = tokenizer.tokenize();

  Parser parser(std::move(tokens));
  auto tree_opt = parser.parse_prog();

  if (!tree_opt) {
    std::cerr << "Parsing failed\n";
    std::exit(EXIT_FAILURE);
  }

  node::NodeProg tree = *tree_opt;

  Generator generator(tree);
  return generator.gen_prog();
}
