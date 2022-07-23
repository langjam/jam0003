#pragma once

#include <string>
#include <map>
#include <vector>

#include "runtime/grid.h"
#include "ast/exprs/expr.h"
#include "ast/instructions/instruction.h"

class State {
   private:
    std::map<std::string, AstExpr> m_variables;

    Grid grid;
   public:
    State();

    void set_variable(std::string name, AstExpr expr) { m_variables.insert(std::make_pair(name, expr)); }

    AstExpr get_variable(std::string name) { return m_variables.at(name); };
};
