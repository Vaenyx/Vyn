#pragma once

#include "tokenization.hpp"
#include <cstddef>
#include <cstdlib>
#include <iostream>
#include <optional>
#include <variant>
#include <vector>

namespace node {

struct NodeType {
  Token type;
};

struct NodeMutability {
  Token mutab;
};

struct NodeExprIntLit {
  Token int_lit;
};

struct NodeExprIdent {
  Token ident;
};

struct NodeExpr {
  std::variant<NodeExprIntLit, NodeExprIdent> var;
};

struct NodeStmtExit {
  NodeExpr expr;
};

struct NodeStmtDecl {
  NodeMutability mutab;
  Token ident;
  NodeType type;
  NodeExpr expr;
};

struct NodeStmt {
  std::variant<NodeStmtExit, NodeStmtDecl> var;
};

struct NodeProg {
  std::vector<NodeStmt> stmts;
};

} // namespace node

class Parser {
public:
  explicit Parser(std::vector<Token> tokens) : m_tokens(std::move(tokens)) {}

  std::optional<node::NodeExpr> parse_expr() {
    if (peek().type == TokenType::_int_lit) {
      Token tok = consume();
      return node::NodeExpr{node::NodeExprIntLit{tok}};
    }

    if (peek().type == TokenType::_ident) {
      Token tok = consume();
      return node::NodeExpr{node::NodeExprIdent{tok}};
    }

    return std::nullopt;
  };

  std::optional<node::NodeStmt> parse_stmt() {

    // exit(expr)
    if (peek().type == TokenType::_exit) {
      consume();

      if (peek().type != TokenType::_open_paren) {
        std::cerr << "Expected '('\n";
        exit(EXIT_FAILURE);
      }
      consume();

      auto expr = parse_expr();
      if (!expr) {
        std::cerr << "Expected expression inside exit()\n";
        exit(EXIT_FAILURE);
      }

      if (peek().type != TokenType::_close_paren) {
        std::cerr << "Expected ')'\n";
        exit(EXIT_FAILURE);
      }
      consume();

      return node::NodeStmt{node::NodeStmtExit{expr.value()}};
    }

    // const x : int = expr
    if (peek().type == TokenType::_const || peek().type == TokenType::_mut) {
      Token mutabTok = consume();

      if (peek().type != TokenType::_ident) {
        std::cerr << "Expected identifier after const\n";
        exit(EXIT_FAILURE);
      }

      Token ident = consume();

      if (peek().type != TokenType::_colon) {
        std::cerr << "Expected ':' after identifier\n";
        exit(EXIT_FAILURE);
      }
      consume();

      if (peek().type != TokenType::_int) {
        std::cerr << "Expected type after ':'\n";
        exit(EXIT_FAILURE);
      }
      Token typeTok = consume();

      if (peek().type != TokenType::_assign) {
        std::cerr << "Expected '='\n";
        exit(EXIT_FAILURE);
      }

      consume();

      auto expr = parse_expr();
      if (!expr) {
        std::cerr << "Expected expression after '='\n";
        exit(EXIT_FAILURE);
      }

      return node::NodeStmt{node::NodeStmtDecl{node::NodeMutability{mutabTok},
                                               ident, node::NodeType{typeTok},
                                               expr.value()}};
    }
    return std::nullopt;
  }

  std::optional<node::NodeProg> parse_prog() {
    node::NodeProg prog;

    while (peek().type != TokenType::_eof) {
      auto stmt = parse_stmt();

      if (stmt) {
        prog.stmts.push_back(stmt.value());
      } else {
        std::cerr << "Invalid statement\n";
        exit(EXIT_FAILURE);
      }

      if (peek().type == TokenType::_newline) {
        consume();
      }
    }

    return prog;
  }

private:
  const Token &peek(size_t ahead = 0) const {
    if (m_idx + ahead >= m_tokens.size()) {
      return m_tokens.back(); // _eof
    }
    return m_tokens.at(m_idx + ahead);
  }

  Token consume() { return m_tokens.at(m_idx++); }

private:
  std::vector<Token> m_tokens;
  size_t m_idx = 0;
};
