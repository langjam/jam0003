#include "combinationvalue.h"
#include <runtime/values/repeatvalue.h>
#include <runtime/values/numbervalue.h>
#include <cassert>

Value::Ptr CombinationType::add(Value::Ptr lhs, Value::Ptr rhs) {
    auto values_copy = dynamic_cast<CombinationValue*>(lhs.get())->values();
    values_copy.push_back(rhs);
    return std::make_shared<CombinationValue>(values_copy);
}

Value::Ptr CombinationType::mul(Value::Ptr lhs, Value::Ptr rhs) {
    if (rhs->is_number()) {
        auto number_value = dynamic_cast<NumberValue*>(rhs.get());
        return std::make_shared<RepeatValue>(lhs, number_value->value());
    } else {
        return ValueType::mul(lhs, rhs);;
    }
}

void CombinationValue::generate_commands(State& state) {
    for (auto v : m_values)
        v->generate_commands(state);
}
