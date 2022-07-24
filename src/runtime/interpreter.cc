#include "interpreter.h"

void Interpreter::run(std::vector<AstInstruction*> instructions) {
    for (const auto &i : instructions) {
        i->run(&m_state);
    }
}
