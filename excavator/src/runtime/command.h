#pragma once

#include <cassert>
#include <vector>

class Simulation;

enum CommandType { NoCommand, Multiple, GoLeft, GoRight, GoUp, GoDown };

class CommandCell {
public:
    CommandCell(CommandType type = CommandType::NoCommand) : m_type(type) {}
    ~CommandCell() {}

    void run(Simulation& sim);
    int to_char();
    void add_command(CommandType type);

    CommandType type() { return m_type; }

private:
    CommandType m_type;
    std::vector<CommandCell> m_commands {};
    size_t m_index { 0 };

    size_t size() { return m_commands.size(); }
};
