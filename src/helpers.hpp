#pragma once

#include <filesystem>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>

std::string get_file_content(const std::string &file_path) {
  std::ifstream input(file_path);
  if (!input) {
    std::cerr << "Failed to open input file\n";
    exit(EXIT_FAILURE);
  }

  std::stringstream contents_stream;
  contents_stream << input.rdbuf();
  return contents_stream.str();
}

void write_to_file(std::string file_path, std::string content) {
  std::ofstream file(file_path);
  if (!file) {
    std::cerr << "Failed to open output file\n";
    exit(EXIT_FAILURE);
  }

  file << content;
  file.close();
}

std::filesystem::path get_tmp_path(const std::string &extensions) {
  auto now = std::chrono::steady_clock::now().time_since_epoch().count();

  return std::filesystem::temp_directory_path() /
         ("vyn_tmp" + std::to_string(now) + "." + extensions);
}
