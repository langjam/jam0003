#pragma once

#include <runtime/values/value.h>

#include <vector>

class NumberType : public ValueType {
   public:
    typedef std::shared_ptr<NumberType> Ptr;

    Value::Ptr mul(Value::Ptr lhs, Value::Ptr rhs) override;
    Value::Ptr add(Value::Ptr lhs, Value::Ptr rhs) override;

    static NumberType& the() {
        if (m_the) return *m_the;
        return *(m_the = new NumberType);
    }

   private:
    NumberType() : ValueType() {}

    static inline NumberType* m_the = nullptr;
};

class NumberValue : public Value {
   public:
    NumberValue(int value) : Value(NumberType::the()), m_value(value) {}

    int value() { return m_value; }
    bool is_number() override { return true; }
    void generate_commands(State &state) override;

   private:
    int m_value;
};
