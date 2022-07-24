#pragma once

class Command {
   public:
    enum Type { Undefined, GoLeft, GoRight, GoUp, GoDown };
    Command(Type type = Undefined) : m_type(type) {}

    Type type() { return m_type; }

   private:
    Type m_type;
};
