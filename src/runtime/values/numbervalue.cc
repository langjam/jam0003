#include "numbervalue.h"

#include <runtime/values/repeatvalue.h>

#include <cassert>
#include <iostream>

Value::Ptr NumberType::add(Value::Ptr lhs, Value::Ptr rhs) {
    auto number_value = dynamic_cast<NumberValue*>(lhs.get());
    if (rhs->is_number()) {
        auto rhs_number_value = dynamic_cast<NumberValue*>(rhs.get());
        return std::make_shared<NumberValue>(number_value->value() * rhs_number_value->value());
    } else {
        return ValueType::add(lhs, rhs);
    }
}

Value::Ptr NumberType::mul(Value::Ptr lhs, Value::Ptr rhs) {
    auto number_value = dynamic_cast<NumberValue*>(lhs.get());
    if (rhs->is_repeat()) {
        auto repeat_value = dynamic_cast<RepeatValue*>(rhs.get());
        return std::make_shared<RepeatValue>(
            repeat_value->value(),
            repeat_value->times() * number_value->value());
    } else if (rhs->is_number()) {
        auto rhs_number_value = dynamic_cast<NumberValue*>(rhs.get());
        return std::make_shared<NumberValue>(number_value->value() * rhs_number_value->value());
    } else {
        return std::make_shared<RepeatValue>(
            rhs,
            number_value->value());
    }
}
