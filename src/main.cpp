#include <cctype>
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
      while (isdigit(str.at(j))) {
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
  if (argc != 3) {
    cerr << "Incorrect usag. Correct usage is...\n";
    cerr << "vyn <input.vyn> <output.c>\n";
    return EXIT_FAILURE;
  }

  string contents;
  {
    stringstream contents_stream;
    fstream input(argv[1], ios::in);
    contents_stream << input.rdbuf();
    contents = contents_stream.str();
  }

  vector<Token> tokens = tokenize(contents);

  {
    fstream file(argv[2], ios::out);
    file << tokens_to_c(tokens);
  }

  return EXIT_SUCCESS;
}
