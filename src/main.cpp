#include <CLI/CLI.hpp>
#include <cctype>
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <optional>
#include <sstream>
#include <vector>

using namespace std;

enum class TokenType { _return, _int_lit, _int, _str, _newline };

struct Token {
  TokenType type;
  optional<string> value;
};

vector<Token> tokenize(const string &str) {
  vector<Token> tokens;
  string buf;

  for (int i = 0; i < str.length(); ++i) {
    char c = str.at(i);

    if (isalpha(c)) {
      buf.clear();
      buf.push_back(c);

      int j = i + 1;
      while (j < str.length() && isalnum(str.at(j))) {
        buf.push_back(str.at(j));
        ++j;
      }

      i = j - 1;

      if (buf == "return") {
        tokens.push_back({.type = TokenType::_return});
      } else if (buf == "int") {
        tokens.push_back({.type = TokenType::_int});
      } else if (buf == "str") {
        tokens.push_back({.type = TokenType::_str});
      } else {
        cerr << "You messed up";
        exit(EXIT_FAILURE);
      }
    }

    else if (isdigit(c)) {
      buf.clear();
      buf.push_back(c);
      int j = i + 1;
      while (j < str.length() && isdigit(str.at(j))) {
        buf.push_back(str.at(j));
        ++j;
      }
      i = j - 1;
      tokens.push_back({.type = TokenType::_int_lit, .value = buf});
    } else if (c == '\n') {
      tokens.push_back({.type = TokenType::_newline});
    } else if (isspace(c)) {
      continue;
    } else {
      cerr << "You messed up";
      exit(EXIT_FAILURE);
    }
  }

  return tokens;
}
string tokens_to_c(const vector<Token> &tokens) {
  stringstream output;

  output << "#include <stdio.h>\n\nint main() {\n";

  for (int i = 0; i < tokens.size(); ++i) {
    const Token &token = tokens.at(i);

    if (token.type == TokenType::_return) {
      if (i + 1 < tokens.size() &&
          tokens.at(i + 1).type == TokenType::_int_lit) {

        if (i + 2 < tokens.size() &&
            tokens.at(i + 2).type == TokenType::_newline) {

          output << "return ";
          output << tokens.at(i + 1).value.value() << ";";
        }
      }
    }
  }

  output << "\n}";
  return output.str();
}

int main(int argc, char *argv[]) {
  CLI::App app{"Vyn compiler"};

  std::string input_file;
  std::string out_file = "a.out";

  bool c_file = false;

  app.add_option("input_file", input_file, "Input file")->required();
  app.add_option("-o,--out", out_file, "Output file");
  app.add_flag("-c,--cfile", c_file, "Creates a c file instead of a binary");

  CLI11_PARSE(app, argc, argv);

  string contents;
  {
    stringstream contents_stream;

    fstream input(input_file, ios::in);
    if (!input) {
      cerr << "Failed to open input file\n";
      return EXIT_FAILURE;
    }

    contents_stream << input.rdbuf();
    contents = contents_stream.str();
  }

  vector<Token> tokens = tokenize(contents);
  string c_content = tokens_to_c(tokens);

  if (c_file) {
    fstream file(out_file, ios::out);
    if (!file) {
      cerr << "Failed to open output file\n";
      return EXIT_FAILURE;
    }

    file << c_content;

  } else {
    string cmd_string = "clang -x c - -o " + out_file;
    FILE *pipe = popen(cmd_string.c_str(), "w");
    fputs(c_content.c_str(), pipe);
    pclose(pipe);
  }

  return EXIT_SUCCESS;
}
