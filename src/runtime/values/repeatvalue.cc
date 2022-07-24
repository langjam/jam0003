#include "repeatvalue.h"
#include <runtime/values/numbervalue.h>

#include <cassert>

Value::Ptr RepeatType::mul(Value::Ptr lhs, Value::Ptr rhs) {
    auto repeat_value = dynamic_cast<RepeatValue*>(lhs.get());
    if (rhs->is_number()) {
        auto number_value = dynamic_cast<NumberValue*>(rhs.get());
        return std::make_shared<RepeatValue>(
            repeat_value->value(),
            repeat_value->times() * number_value->value());
    } else {
        return ValueType::add(lhs, rhs);
    }
}

void RepeatValue::generate_commands(State &state) {
    for (int i = 0; i < m_times; ++i)
        m_value->generate_commands(state);
}
