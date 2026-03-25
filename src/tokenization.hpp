#pragma once

#include <cctype>
#include <cstdlib>
#include <iostream>
#include <optional>
#include <string>
#include <vector>

enum class TokenType { _return, _int_lit, _int, _str, _newline };

struct Token {
  TokenType type;
  std::optional<std::string> value;
};

class Tokenizer {
public:
  inline explicit Tokenizer(std::string src) : m_src(std::move(src)) {};

  inline std::vector<Token> tokenize() {
    std::vector<Token> tokens;
    std::string buf;

    while (auto c = peek()) {
      if (std::isalpha(static_cast<unsigned char>(*c))) {
        buf.clear();
        buf.push_back(consume());

        auto n = peek();
        while (n && std::isalnum(static_cast<unsigned char>(*n))) {
          buf.push_back(consume());
          n = peek();
        }

        if (buf == "return") {
          tokens.push_back({.type = TokenType::_return});
        } else {
          std::cerr << "Invalid identifier: " << buf << "\n";
          exit(EXIT_FAILURE);
        }
      }

      else if (std::isdigit(static_cast<unsigned char>(*c))) {
        buf.clear();
        buf.push_back(consume());

        auto n = peek();
        while (n && std::isdigit(static_cast<unsigned char>(*n))) {
          buf.push_back(consume());
          n = peek();
        }
        tokens.push_back({.type = TokenType::_int_lit, .value = buf});
      }

      else if (*c == '\n') {
        consume();
        tokens.push_back({.type = TokenType::_newline});
      }

      else if (std::isspace(static_cast<unsigned char>(*c))) {
        consume();
      }

      else {
        std::cerr << "Unexpected character: " << *c << "\n";
        exit(EXIT_FAILURE);
      }
    }
    m_idx = 0;
    return tokens;
  }

private:
  [[nodiscard]] std::optional<char> peek(int ahead = 0) const {
    if (m_idx + ahead >= m_src.length()) {
      return {};
    }
    return m_src.at(m_idx + ahead);
  }

  char consume() { return m_src.at(m_idx++); }

  const std::string m_src;
  int m_idx = 0;
};
