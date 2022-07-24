#include "mulexpr.h"

#include <cassert>

Value::Ptr AstMulExpr::eval(State& state) {
    auto l = lhs()->eval(state);
    auto r = rhs()->eval(state);
    auto result = l->type().mul(l, r);
    if (!result) {
        // FIXME: Error
        assert(0);
    }
    return result;
}
