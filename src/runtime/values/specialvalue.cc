#include "specialvalue.h"

#include <runtime/command.h>
#include <runtime/state.h>

#include <cassert>

void SpecialValue::generate_commands(State& state) {
    Command command;
    switch (m_type) {
    case GoLeft:
        command = Command(Command::Type::GoLeft);
        break;
    case GoRight:
        command = Command(Command::Type::GoRight);
        break;
    case GoUp:
        command = Command(Command::Type::GoUp);
        break;
    case GoDown:
        command = Command(Command::Type::GoDown);
        break;
    default:
        assert(0);
    }
    state.add_command(command);
}
