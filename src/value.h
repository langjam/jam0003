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
  public:
    CommandKind tag;

    void display();
};
