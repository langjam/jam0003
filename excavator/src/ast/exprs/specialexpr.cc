#include "specialexpr.h"
#include <runtime/values/specialvalue.h>

Value::Ptr AstSpecialExpr::eval(State& state) {
    (void)state;
    SpecialValue::Type type;
    switch (m_type) {
    case GoLeft:
        type = SpecialValue::Type::GoLeft;
        break;
    case GoRight:
        type = SpecialValue::Type::GoRight;
        break;
    case GoUp:
        type = SpecialValue::Type::GoUp;
        break;
    case GoDown:
        type = SpecialValue::Type::GoDown;
        break;
    }
    return std::make_shared<SpecialValue>(type);
}
