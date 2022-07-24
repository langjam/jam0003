#pragma once

#include <iostream>
#include <map>
#include <string>
#include <vector>

// #include <runtime/grid.h>
#include <ast/exprs/expr.h>
#include <ast/instructions/instruction.h>

class State {
   public:
    State() {}
    ~State() = default;

    Value::Ptr get_variable(std::string name) { return m_variables.at(name); };
    void set_variable(std::string name, Value::Ptr expr) {
        m_variables.insert(std::make_pair(name, expr));
    }
    // Grid& grid() { return m_grid; }

   private:
    std::map<std::string, Value::Ptr> m_variables;
    // Grid m_grid;
};
