#include "addexpr.h"

#include <cassert>

Value::Ptr AstAddExpr::eval(State& state) {
    auto l = lhs()->eval(state);
    auto r = rhs()->eval(state);
    auto result = l->type().add(l, r);
    if (!result) {
        // FIXME: Error
        assert(0);
    }
    return result;
}
