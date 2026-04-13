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
  _open_curly_paren,
  _close_curly_paren,
  _ident,
  _assign,
  _const,
  _mut,
  _colon,
  _if,
  _else,
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
  explicit Tokenizer(std::string src) : m_src(std::move(src)) {}

  std::vector<Token> tokenize() {
    std::vector<Token> tokens;
    std::string buf;

    while (auto c = peek()) {
      size_t start_line = m_line;
      size_t start_col = m_col;

      // ======================
      // COMMENTS
      // ======================
      if (*c == '/' && peek(1)) {
        if (*peek(1) == '/') {
          consume(); // '/'
          consume(); // '/'

          while (peek() && *peek() != '\n') {
            consume();
          }
          continue;
        }

        if (*peek(1) == '*') {
          consume(); // '/'
          consume(); // '*'

          while (peek()) {
            if (*peek() == '*' && peek(1) && *peek(1) == '/') {
              consume();
              consume();
              break;
            }
            consume(); // IMPORTANT
          }
          continue;
        }
      }

      // ======================
      // IDENTIFIERS / KEYWORDS
      // ======================
      if (std::isalpha(static_cast<unsigned char>(*c))) {
        buf.clear();
        buf.push_back(consume());

        while (peek() && std::isalnum(static_cast<unsigned char>(*peek()))) {
          buf.push_back(consume());
        }

        tokens.push_back(buf_to_token(buf, start_line, start_col));
        continue;
      }

      // ======================
      // NUMBERS
      // ======================
      if (std::isdigit(static_cast<unsigned char>(*c))) {
        buf.clear();
        buf.push_back(consume());

        while (peek() && std::isdigit(static_cast<unsigned char>(*peek()))) {
          buf.push_back(consume());
        }

        tokens.emplace_back(TokenType::_int_lit, buf, start_line, start_col);
        continue;
      }

      // ======================
      // SINGLE CHAR TOKENS
      // ======================
      auto tok = char_to_token(*c, start_line, start_col);

      consume(); // ALWAYS consume here

      if (tok) {
        tokens.push_back(*tok);
      }
    }

    tokens.emplace_back(TokenType::_eof, std::nullopt, m_line, m_col);
    return tokens;
  }

private:
  std::optional<char> peek(size_t ahead = 0) const {
    if (m_idx + ahead >= m_src.size())
      return std::nullopt;
    return m_src[m_idx + ahead];
  }

  char consume() {
    char c = m_src[m_idx++];

    if (c == '\n') {
      m_line++;
      m_col = 1;
    } else {
      m_col++;
    }

    return c;
  }

  Token buf_to_token(const std::string &buf, size_t line, size_t col) {
    if (buf == "return")
      return {TokenType::_return, std::nullopt, line, col};
    if (buf == "exit")
      return {TokenType::_exit, std::nullopt, line, col};
    if (buf == "const")
      return {TokenType::_const, std::nullopt, line, col};
    if (buf == "mut")
      return {TokenType::_mut, std::nullopt, line, col};
    if (buf == "if")
      return {TokenType::_if, std::nullopt, line, col};
    if (buf == "else")
      return {TokenType::_else, std::nullopt, line, col};
    if (buf == "int")
      return {TokenType::_int, std::nullopt, line, col};

    return {TokenType::_ident, buf, line, col};
  }

  std::optional<Token> char_to_token(char c, size_t line, size_t col) {
    switch (c) {
    case '=':
      return Token{TokenType::_assign, std::nullopt, line, col};
    case ':':
      return Token{TokenType::_colon, std::nullopt, line, col};
    case '(':
      return Token{TokenType::_open_paren, std::nullopt, line, col};
    case ')':
      return Token{TokenType::_close_paren, std::nullopt, line, col};
    case '{':
      return Token{TokenType::_open_curly_paren, std::nullopt, line, col};
    case '}':
      return Token{TokenType::_close_curly_paren, std::nullopt, line, col};
    case '\n':
      return Token{TokenType::_newline, std::nullopt, line, col};
    }

    if (std::isspace(static_cast<unsigned char>(c))) {
      return std::nullopt;
    }

    std::cerr << "Unexpected character at line " << line << ", col " << col
              << ": " << c << "\n";
    std::exit(EXIT_FAILURE);
  }

private:
  const std::string m_src;
  size_t m_idx = 0;
  size_t m_line = 1;
  size_t m_col = 1;
};
