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

struct NodeStmt;

struct NodeStmtBlock {
  std::vector<NodeStmt> stmts;
};

struct NodeStmtIf {
  NodeExpr condition;
  NodeStmtBlock thenBlock;
  std::optional<NodeStmtBlock> elseBlock;
};

struct NodeStmt {
  std::variant<NodeStmtExit, NodeStmtDecl, NodeStmtIf, NodeStmtBlock> var;
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

    if (peek().type == TokenType::_if) {
      consume();

      auto condition = parse_expr();
      if (!condition) {
        std::cerr << "Expected condition after if at line " << peek().m_line
                  << ", col " << peek().m_col << "\n";

        exit(EXIT_FAILURE);
      }

      auto thenBlock = parse_block();

      while (peek().type == TokenType::_newline) {
        consume();
      }

      std::optional<node::NodeStmtBlock> elseBlock;

      if (peek().type == TokenType::_else) {
        consume();

        auto blk = parse_block();

        elseBlock = blk;
      }

      return node::NodeStmt{
          node::NodeStmtIf{condition.value(), thenBlock, elseBlock}};
    }
    // exit(expr)
    if (peek().type == TokenType::_exit) {
      consume();

      if (peek().type != TokenType::_open_paren) {
        std::cerr << "Expected '(' at line " << peek().m_line << ", col "
                  << peek().m_col << "\n";
        exit(EXIT_FAILURE);
      }
      consume();

      auto expr = parse_expr();
      if (!expr) {
        std::cerr << "Expected expression inside exit() at line "
                  << peek().m_line << ", col " << peek().m_col << "\n";

        exit(EXIT_FAILURE);
      }

      if (peek().type != TokenType::_close_paren) {
        std::cerr << "Expected ')' at line " << peek().m_line << ", col "
                  << peek().m_col << "\n";

        exit(EXIT_FAILURE);
      }
      consume();

      return node::NodeStmt{node::NodeStmtExit{expr.value()}};
    }

    // const x : int = expr
    if (peek().type == TokenType::_const || peek().type == TokenType::_mut) {
      Token mutabTok = consume();

      if (peek().type != TokenType::_ident) {
        std::cerr << "Expected identifier after " << peek().value.value_or("")
                  << " at line " << peek().m_line << ", col " << peek().m_col
                  << "\n";

        exit(EXIT_FAILURE);
      }

      Token ident = consume();

      if (peek().type != TokenType::_colon) {
        std::cerr << "Expected ':' after identifier at line " << peek().m_line
                  << ", col " << peek().m_col << "\n";

        exit(EXIT_FAILURE);
      }
      consume();

      if (peek().type != TokenType::_int) {
        std::cerr << "Expected type after ':' at line " << peek().m_line
                  << ", col " << peek().m_col << "\n";

        exit(EXIT_FAILURE);
      }
      Token typeTok = consume();

      if (peek().type != TokenType::_assign) {
        std::cerr << "Expected '=' at line " << peek().m_line << ", col "
                  << peek().m_col << "\n";

        exit(EXIT_FAILURE);
      }

      consume();

      auto expr = parse_expr();
      if (!expr) {
        std::cerr << "Expected expression after '=' at line " << peek().m_line
                  << ", col " << peek().m_col << "\n";

        exit(EXIT_FAILURE);
      }

      return node::NodeStmt{node::NodeStmtDecl{node::NodeMutability{mutabTok},
                                               ident, node::NodeType{typeTok},
                                               expr.value()}};
    }
    return std::nullopt;
  }

  node::NodeStmtBlock parse_block() {
    while (peek().type == TokenType::_newline)
      consume();

    if (peek().type != TokenType::_open_curly_paren) {
      std::cerr << "Expected '{' before block at line " << peek().m_line
                << ", col " << peek().m_col << "\n";

      exit(EXIT_FAILURE);
    }

    consume();

    node::NodeStmtBlock block;

    while (peek().type != TokenType::_close_curly_paren) {

      while (peek().type == TokenType::_newline)
        consume();

      if (peek().type == TokenType::_close_curly_paren)
        break;

      auto stmt = parse_stmt();
      if (!stmt) {
        std::cerr << "Invalid statement at line " << peek().m_line << ", col "
                  << peek().m_col << "\n";
        exit(EXIT_FAILURE);
      }

      block.stmts.push_back(stmt.value());

      if (peek().type == TokenType::_newline)
        consume();
    }

    while (peek().type == TokenType::_newline)
      consume();

    if (peek().type != TokenType::_close_curly_paren) {
      std::cerr << "Expected '}' after block at line " << peek().m_line
                << ", col " << peek().m_col << "\n";

      exit(EXIT_FAILURE);
    }

    consume();

    return block;
  }

  std::optional<node::NodeProg> parse_prog() {
    node::NodeProg prog;

    while (peek().type != TokenType::_eof) {

      while (peek().type == TokenType::_newline) {
        consume();
      }

      if (peek().type == TokenType::_eof)
        break;

      auto stmt = parse_stmt();
      if (stmt) {
        prog.stmts.push_back(stmt.value());
      } else {
        std::cerr << "Invalid statement at line " << peek().m_line << ", col "
                  << peek().m_col << "\n";

        exit(EXIT_FAILURE);
      }

      if (peek().type == TokenType::_newline) {
        consume();
      } else if (peek().type != TokenType::_eof) {
        std::cerr << "Expected newline after statement at line "
                  << peek().m_line << ", col " << peek().m_col << "\n";

        exit(EXIT_FAILURE);
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
