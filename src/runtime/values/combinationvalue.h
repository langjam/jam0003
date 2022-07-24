#pragma once

#include <runtime/values/value.h>

#include <vector>

class CombinationType : public ValueType {
   public:
    Value::Ptr add(Value::Ptr lhs, Value::Ptr rhs) override;
    Value::Ptr mul(Value::Ptr lhs, Value::Ptr rhs) override;

    static CombinationType& the() {
        if (m_the) return *m_the;
        return *(m_the = new CombinationType);
    }

   private:
    CombinationType() : ValueType() {}

    static inline CombinationType* m_the = nullptr;
};

class CombinationValue : public Value {
   public:
    CombinationValue(std::vector<Value::Ptr> values)
        : Value(CombinationType::the()), m_values(values) {}

    std::vector<Value::Ptr> values() { return m_values; }

    bool is_assemble() override { return true; }

   private:
    std::vector<Value::Ptr> m_values;
};