#include "generateinstruction.h"

void AstGenerateInstruction::run(State& state) {
    auto to_generate_from = m_value_expr->eval(state);
    to_generate_from->generate_commands(state);
    for (auto c : state.commands()) {
        std::cout << "Command no. " << c.type() << std::endl;
    }
}