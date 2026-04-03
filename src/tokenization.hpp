#pragma once

#include <cctype>
#include <cstdlib>
#include <iostream>
#include <optional>
#include <string>
#include <vector>

enum class TokenType {
  _return,
  _int_lit,
  _int,
  _str,
  _newline,
  _exit,
  _eof,
  _open_paren,
  _close_paren,
  _ident,
  _assign,
  _const,
  _colon
};

struct Token {
  TokenType type;
  std::optional<std::string> value;

  Token(TokenType t, std::optional<std::string> v = std::nullopt)
      : type(t), value(std::move(v)) {}
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
          tokens.push_back(Token{TokenType::_return});
        } else if (buf == "exit") {
          tokens.push_back(Token{TokenType::_exit});
        } else if (buf == "const") {
          tokens.push_back(Token{TokenType::_const});
        } else if (buf == "int") {
          tokens.push_back(Token{TokenType::_int});
        } else {
          tokens.push_back(Token{TokenType::_ident, buf});
        }
        continue;
      }

      if (std::isdigit(static_cast<unsigned char>(*c))) {
        buf.clear();
        buf.push_back(consume());

        auto n = peek();
        while (n && std::isdigit(static_cast<unsigned char>(*n))) {
          buf.push_back(consume());
          n = peek();
        }
        tokens.push_back(Token{TokenType::_int_lit, buf});
        continue;
      }

      if (c == '=') {
        consume();
        tokens.push_back(Token{TokenType::_assign});
        continue;
      }

      if (c == ':') {
        consume();
        tokens.push_back(Token{TokenType::_colon});
        continue;
      }

      if (c == '(') {
        consume();
        tokens.push_back(Token{TokenType::_open_paren});
        continue;
      }

      if (c == ')') {
        consume();
        tokens.push_back(Token{TokenType::_close_paren});
        continue;
      }

      if (c == '\n') {
        consume();
        tokens.push_back(Token{TokenType::_newline});
        continue;
      }

      if (std::isspace(static_cast<unsigned char>(*c))) {
        consume();
        continue;
      }

      std::cerr << "Unexpected character: " << *c << "\n";
      exit(EXIT_FAILURE);
    }
    tokens.push_back(Token{TokenType::_eof});
    m_idx = 0;
    return tokens;
  }

private:
  std::optional<char> peek(size_t ahead = 0) const {
    if (m_idx + ahead >= m_src.size()) {
      return std::nullopt;
    }
    return m_src[m_idx + ahead];
  }
  inline char consume() { return m_src.at(m_idx++); }

  const std::string m_src;
  size_t m_idx = 0;
};
