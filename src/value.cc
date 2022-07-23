#include <iostream>

#include "value.h"

Value::Value() {
    return;
}

void Command::display() {
    char c;
    switch (tag) {    
        case Right:
            c = 'r';
            break;
        case Left:
            c = 'l';
            break;
        case Up:
            c = '^';
            break;
        case Down:
            c = ',';
            break;
    }
    std::cout << c;
}
