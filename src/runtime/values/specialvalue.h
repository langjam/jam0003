#pragma once

#include <runtime/values/value.h>

class SpecialType : public ValueType {
   public:
    static SpecialType& the() {
        if (m_the) return *m_the;
        return *(m_the = new SpecialType);
    }

   private:
    SpecialType() : ValueType() {}

    static inline SpecialType* m_the = nullptr;
};

class SpecialValue : public Value {
   public:
    enum Type { GoLeft = 1, GoRight, GoUp, GoDown };
    SpecialValue(Type type) : Value(SpecialType::the()), m_type(type) {}
    bool is_special() override { return true; }

   private:
    Type m_type;
};
