#include "value.h"

#include <iostream>

Value::Value() { return; }

void Command::display() { std::cout << c_; }

void Command::set_char(char c) { c_ = c; }

char Command::character() { return c_; }

CommandKind Command::tag() { return tag_; }
