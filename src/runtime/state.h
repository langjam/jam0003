#pragma once

#include <iostream>
#include <map>
#include <string>
#include <vector>

// #include <runtime/grid.h>
#include <ast/exprs/expr.h>
#include <ast/instructions/instruction.h>
#include <runtime/command.h>

class State {
   public:
    State() {}
    ~State() = default;

    Value::Ptr get_variable(std::string name) { return m_variables.at(name); };
    void set_variable(std::string name, Value::Ptr expr) {
        m_variables.insert(std::make_pair(name, expr));
    }

    void add_command(Command command) {
        m_commands.push_back(command);
    }
    // Grid& grid() { return m_grid; }

    const std::vector<Command>& commands() { return m_commands; }

   private:
    std::map<std::string, Value::Ptr> m_variables;
    std::vector<Command> m_commands;
    // Grid m_grid;
};
