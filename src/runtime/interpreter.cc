#include "interpreter.h"

void Interpreter::run_code() {
    for (const auto &instruction : m_instructions)
        instruction->run(m_state);
}
