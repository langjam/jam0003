#include "numberexpr.h"
#include <runtime/values/numbervalue.h>

Value::Ptr AstNumberExpr::eval(State& state) {
    (void)state;
    return std::make_shared<NumberValue>(m_value);
}
