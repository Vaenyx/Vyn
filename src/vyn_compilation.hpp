#pragma once

#include "generation.hpp"
#include "parser.hpp"
#include "tokenization.hpp"
#include <string>

std::string vyn_to_c(const std::string &input) {
  Tokenizer tokenizer(input);

  std::vector<Token> tokens = tokenizer.tokenize();

  Parser parser(std::move(tokens));
  std::vector<node::NodeStatement> tree = parser.parse();

  if (tree.size() == 0) {
    std::cerr << "No exit statement found\n";
    exit(EXIT_FAILURE);
  }

  Generator generator(tree);
  return generator.generate();
};
