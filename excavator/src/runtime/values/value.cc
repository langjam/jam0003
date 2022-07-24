#include "value.h"

#include <memory>
#include <iostream>
#include <runtime/values/combinationvalue.h>
#include <runtime/values/numbervalue.h>
#include <runtime/values/repeatvalue.h>

Value::Ptr ValueType::add(Value::Ptr lhs, Value::Ptr rhs) {
    auto values = { lhs, rhs };
    return std::make_shared<CombinationValue>(values);
}

Value::Ptr ValueType::mul(Value::Ptr lhs, Value::Ptr rhs) {
    if (rhs->is_number()) {
        auto number_value = dynamic_cast<NumberValue*>(rhs.get());
        return std::make_shared<RepeatValue>(
            lhs,
            number_value->value());
    } else {
        return nullptr;
    }
}
