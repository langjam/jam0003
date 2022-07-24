#include "command.h"
#include <runtime/simulation.h>

void CommandCell::run(Simulation& sim) {
    switch (m_type) {
    case NoCommand:
        sim.set_running(false);
        break;
    case Multiple:
        m_index %= size();
        m_commands[m_index].run(sim);
        break;
    case GoLeft:
        sim.move(-1, 0);
        sim.paint();
        break;
    case GoRight:
        sim.move(1, 0);
        sim.paint();
        break;
    case GoUp:
        sim.move(0, -1);
        sim.paint();
        break;
    case GoDown:
        sim.move(0, 1);
        sim.paint();
        break;
    default:
        assert(0);
    }
}

int CommandCell::to_char() {
    switch (m_type) {
    case GoLeft:
        return '<';
    case GoRight:
        return '>';
    case GoUp:
        return '^';
    case GoDown:
        return ',';
    case NoCommand:
        return ' ';
    case Multiple:
        return '*';
    default:
        assert(0);
    }
}

void CommandCell::add_command(CommandType type) {
    m_commands.push_back(type);
    if (size() == 1) {
        m_type = type;
    } else {
        m_type = Multiple;
    }
}
