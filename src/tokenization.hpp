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
  _mut,
  _colon
};

struct Token {
  TokenType type;
  std::optional<std::string> value;

  size_t m_line;
  size_t m_col;

  Token(TokenType t, std::optional<std::string> v, size_t line, size_t col)
      : type(t), value(std::move(v)), m_line(line), m_col(col) {}
};

class Tokenizer {
public:
  inline explicit Tokenizer(std::string src) : m_src(std::move(src)) {};

  inline std::vector<Token> tokenize() {
    std::vector<Token> tokens;
    std::string buf;

    while (auto c = peek()) {
      size_t start_line = m_line;
      size_t start_col = m_col;

      if (std::isalpha(static_cast<unsigned char>(*c))) {
        buf.clear();
        buf.push_back(consume());

        auto n = peek();
        while (n && std::isalnum(static_cast<unsigned char>(*n))) {
          buf.push_back(consume());
          n = peek();
        }

        tokens.push_back(buf_to_token(buf, start_line, start_col));
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
        tokens.push_back(
            Token{TokenType::_int_lit, buf, start_line, start_col});
        continue;
      }

      std::optional<Token> char_tok = char_to_token(*c);
      if (char_tok) {
        tokens.push_back(*char_tok);
      }
    }

    tokens.push_back(Token{TokenType::_eof, std::nullopt, m_line, m_col});
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

  Token buf_to_token(const std::string &buf, size_t line, size_t col) {
    if (buf == "return") {
      return Token{TokenType::_return, std::nullopt, line, col};
    } else if (buf == "exit") {
      return Token{TokenType::_exit, std::nullopt, line, col};
    } else if (buf == "const") {
      return Token{TokenType::_const, std::nullopt, line, col};
    } else if (buf == "mut") {
      return Token{TokenType::_mut, std::nullopt, line, col};
    } else if (buf == "int") {
      return Token{TokenType::_int, std::nullopt, line, col};
    } else {
      return Token{TokenType::_ident, buf, line, col};
    }
  }

  std::optional<Token> char_to_token(char c) {
    size_t line = m_line;
    size_t col = m_col;

    if (c == '=') {
      consume();
      return Token{TokenType::_assign, std::nullopt, line, col};
    }

    if (c == ':') {
      consume();
      return Token{TokenType::_colon, std::nullopt, line, col};
    }

    if (c == '(') {
      consume();
      return Token{TokenType::_open_paren, std::nullopt, line, col};
    }

    if (c == ')') {
      consume();
      return Token{TokenType::_close_paren, std::nullopt, line, col};
    }

    if (c == '\n') {
      consume();
      return Token{TokenType::_newline, std::nullopt, line, col};
    }

    if (std::isspace(static_cast<unsigned char>(c))) {
      consume();
      return std::nullopt;
    }

    std::cerr << "Unexpected character at line " << line << ", col " << col
              << ": " << c << "\n";
    exit(EXIT_FAILURE);
  }
  inline char consume() {
    char c = m_src[m_idx++];

    if (c == '\n') {
      m_line++;
      m_col = 1;
    } else {
      m_col++;
    }

    return c;
  }

  const std::string m_src;
  size_t m_idx = 0;
  size_t m_line = 1;
  size_t m_col = 1;
};
