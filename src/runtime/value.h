#pragma once

class Value {
   public:
    Value();

    virtual void display() = 0;
};

enum CommandKind {
    Right,
    Left,
    Up,
    Down,
};

class Command : public Value {
   private:
    CommandKind tag_;
    char c_;

   public:
    void set_char(char c);

    char character();

    CommandKind tag();

    void display();
};
