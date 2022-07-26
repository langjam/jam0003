#include "interpreter.h"
#include <runtime/simulation.h>

void Interpreter::run_code() {
    for (const auto &instruction : m_instructions)
        instruction->run(m_state);
}

void Interpreter::run_simulation() {
    Simulation simulation(m_state.commands());
    simulation.run();
}
