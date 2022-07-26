#include "variableexpr.h"
#include <runtime/state.h>

Value::Ptr AstVariableExpr::eval(State& state) {
    // FIXME: This should check whether the variable even exists
    return state.get_variable(m_name);
}
