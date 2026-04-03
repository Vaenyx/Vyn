#pragma once

#include "parser.hpp"
#include <sstream>
#include <string>

template <class... Ts> struct Overloaded : Ts... {
  using Ts::operator()...;
};

template <class... Ts> Overloaded(Ts...) -> Overloaded<Ts...>;

class Generator {
public:
  explicit Generator(node::NodeProg prog) : m_prog(std::move(prog)) {}
  [[nodiscard]] std::string gen_expr(const node::NodeExpr &expr) const {
    return std::visit(Overloaded{[](const node::NodeExprIntLit &v) {
                                   return v.int_lit.value.value();
                                 },
                                 [](const node::NodeExprIdent &v) {
                                   return v.ident.value.value();
                                 }},
                      expr.var);
  }

  [[nodiscard]] std::string gen_stmt(const node::NodeStmt &stmt) const {
    std::stringstream res;

    std::visit(Overloaded{[&](const node::NodeStmtExit &v) {
                            res << "exit(";
                            res << gen_expr(v.expr);
                            res << ");\n";
                          },
                          [&](const node::NodeStmtConst &v) {
                            res << "const ";
                            res << token_type_to_string(v.type.type.type);
                            res << " ";
                            res << v.ident.value.value();
                            res << " = ";
                            res << gen_expr(v.expr);
                            res << ";\n";
                          }},
               stmt.var);

    return res.str();
  }

  [[nodiscard]] std::string gen_prog() const {
    std::stringstream output;

    output << "#include <stdlib.h>\n\n";
    output << "int main() {\n";

    for (const node::NodeStmt &stmt : m_prog.stmts) {
      output << gen_stmt(stmt);
    }
    output << "return 0;\n";
    output << "}\n";

    return output.str();
  }

private:
  static std::string token_type_to_string(TokenType t) {
    switch (t) {
    case TokenType::_int:
      return "int";
    default:
      return "/*unknown*/";
    }
  }

  node::NodeProg m_prog;
};
