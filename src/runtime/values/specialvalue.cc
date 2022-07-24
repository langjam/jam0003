#include "specialvalue.h"

#include <runtime/command.h>
#include <runtime/state.h>

#include <cassert>

void SpecialValue::generate_commands(State& state) {
    CommandCell command;
    switch (m_type) {
    case GoLeft:
        command = CommandCell(CommandType::GoLeft);
        break;
    case GoRight:
        command = CommandCell(CommandType::GoRight);
        break;
    case GoUp:
        command = CommandCell(CommandType::GoUp);
        break;
    case GoDown:
        command = CommandCell(CommandType::GoDown);
        break;
    default:
        assert(0);
    }
    state.add_command(command);
}
