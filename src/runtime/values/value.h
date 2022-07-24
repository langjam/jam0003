#pragma once

#include <memory>

class ValueType;

class Value {
   public:
    typedef std::shared_ptr<Value> Ptr;
    Value(ValueType& type) : m_type(type) {}
    ~Value() {}

    virtual bool is_number() { return false; }
    virtual bool is_special() { return false; }
    virtual bool is_assemble() { return false; }
    virtual bool is_repeat() { return false; }

    ValueType& type() { return m_type; }
   private:
    ValueType& m_type;
};

class ValueType {
   public:
    virtual Value::Ptr add(Value::Ptr lhs, Value::Ptr rhs);
    virtual Value::Ptr mul(Value::Ptr lhs, Value::Ptr rhs);

   protected:
    ValueType() {}
};
