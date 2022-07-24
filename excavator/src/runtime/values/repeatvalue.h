#pragma once

#include <runtime/values/value.h>

#include <vector>

class RepeatType : public ValueType {
   public:
    Value::Ptr mul(Value::Ptr lhs, Value::Ptr rhs) override;

    static RepeatType& the() {
        if (m_the) return *m_the;
        return *(m_the = new RepeatType);
    }

   private:
    RepeatType() : ValueType() {}

    static inline RepeatType* m_the = nullptr;
};

class RepeatValue : public Value {
   public:
    RepeatValue(Value::Ptr value, int times)
        : Value(RepeatType::the()), m_value(value), m_times(times) {}

    Value::Ptr value() { return m_value; }
    int times() { return m_times; }

    bool is_repeat() override { return true; }
    void generate_commands(State &state) override;

   private:
    Value::Ptr m_value;
    int m_times;
};