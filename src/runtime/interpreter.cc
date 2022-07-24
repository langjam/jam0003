#include "interpreter.h"

void Interpreter::run() {
    for (const auto &instruction : m_instructions)
        instruction->run(m_state);
}
