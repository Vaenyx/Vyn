#pragma once

#include "parser.hpp"
#include <sstream>
#include <string>

class Generator {
public:
  explicit Generator(std::vector<node::NodeStatement> statements)
      : m_statements(std::move(statements)) {}

  [[nodiscard]] std::string generate() const {
    std::stringstream output;

    output << "#include <stdlib.h>\n\n";
    output << "int main() {\n";

    for (const auto &stmt : m_statements) {

      if (std::holds_alternative<node::NodeExit>(stmt.statement)) {
        const auto &s = std::get<node::NodeExit>(stmt.statement);

        output << "exit(" << s.expr.int_lit.value.value() << ");\n";
      }

      else if (std::holds_alternative<node::NodeReturn>(stmt.statement)) {
        const auto &s = std::get<node::NodeReturn>(stmt.statement);

        output << "return " << s.expr.int_lit.value.value() << ";\n";
      }
    }

    output << "}\n";

    return output.str();
  }

private:
  std::vector<node::NodeStatement> m_statements;
};
