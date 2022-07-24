struct s {
    int x;
    char y;
} s = { 1, 2 };

int foo() {
    return s.x;
}

int bar() {
    return foo();
}
