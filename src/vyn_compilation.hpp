#pragma once

#include "generation.hpp"
#include "helpers.hpp"
#include "parser.hpp"
#include "tokenization.hpp"

#include <cstdlib>
#include <filesystem>
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

std::string format_c(const std::string &c_code) {
  std::filesystem::path tmp = get_tmp_path("c");
  write_to_file(tmp, c_code);

  std::string cmd = "clang-format " + tmp.string() + " -i";

  std::system(cmd.c_str());

  std::string formatted_c_code = get_file_content(tmp.string());
  return formatted_c_code;
}

void compile_c(std::string c_code, std::string output_path) {
  std::filesystem::path tmp = get_tmp_path("c");

  write_to_file(tmp, c_code);

  std::string cmd = "clang \"" + tmp.string() + "\" -o \"" + output_path + "\"";

  int result = std::system(cmd.c_str());

  if (result != 0) {
    std::cerr << "Compilation failed\n";
    exit(EXIT_FAILURE);
  }

  std::filesystem::remove(tmp);
}
