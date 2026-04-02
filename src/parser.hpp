#pragma once

#include "tokenization.hpp"
#include <cstddef>
#include <cstdlib>
#include <iostream>
#include <optional>
#include <variant>
#include <vector>

namespace node {

struct NodeExpr {
  Token int_lit;
};

struct NodeExit {
  NodeExpr expr;
};

struct NodeReturn {
  NodeExpr expr;
};

struct NodeStatement {
  std::variant<NodeExit, NodeReturn> statement;
};

} // namespace node

class Parser {
public:
  explicit Parser(std::vector<Token> tokens) : m_tokens(std::move(tokens)) {}

  // Parse an integer literal expression
  std::optional<node::NodeExpr> parse_expr() {
    if (peek().type == TokenType::_int_lit) {
      return node::NodeExpr{consume()};
    }
    return {};
  }

  std::vector<node::NodeStatement> parse() {
    std::vector<node::NodeStatement> statements;
    // loop untill eof
    while (peek().type != TokenType::_eof) {

      // exit keyword
      if (peek().type == TokenType::_exit) {
        consume();

        if (auto node_expr = parse_expr(); node_expr.has_value()) {
          statements.push_back(
              node::NodeStatement{node::NodeExit{node_expr.value()}});
        } else {
          std::cerr << "Invalid expression after 'exit'\n";
          exit(EXIT_FAILURE);
        }
      }

      // return keyword
      else if (peek().type == TokenType::_return) {
        consume();

        if (auto node_expr = parse_expr(); node_expr.has_value()) {
          statements.push_back(
              node::NodeStatement{node::NodeReturn{node_expr.value()}});
        } else {
          std::cerr << "Invalid expression after 'return'\n";
          exit(EXIT_FAILURE);
        }
      }

      else {
        consume();
        continue;
      }

      // newlines/eof
      if (peek().type == TokenType::_newline) {
        consume();
      } else if (peek().type != TokenType::_eof) {
        std::cerr << "Expected newline or EOF\n";
        exit(EXIT_FAILURE);
      }
    }

    m_idx = 0;
    return statements;
  }

private:
  const Token &peek(size_t ahead = 0) const {
    return m_tokens.at(m_idx + ahead);
  }

  Token consume() { return m_tokens.at(m_idx++); }

private:
  std::vector<Token> m_tokens;
  size_t m_idx = 0;
};
