#pragma once

#include <chrono>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>

std::string get_file_content(std::string &file_path) {
  std::string contents;
  {
    std::stringstream contents_stream;

    std::fstream input(file_path, std::ios::in);
    if (!input) {
      std::cerr << "Failed to open input file\n";
      exit(EXIT_FAILURE);
    }

    contents_stream << input.rdbuf();
    return contents_stream.str();
  }
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

void compile_c(std::string c_code, std::string output_path) {
  auto now = std::chrono::steady_clock::now().time_since_epoch().count();

  std::filesystem::path tmp = std::filesystem::temp_directory_path() /
                              ("vyn_tmp_" + std::to_string(now) + ".c");

  write_to_file(tmp, c_code);

  std::string cmd = "clang \"" + tmp.string() + "\" -o \"" + output_path + "\"";

  int result = std::system(cmd.c_str());

  if (result != 0) {
    std::cerr << "Compilation failed\n";
    exit(EXIT_FAILURE);
  }

  std::filesystem::remove(tmp);
}
