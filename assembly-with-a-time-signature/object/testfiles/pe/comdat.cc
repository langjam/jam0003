typedef int (*f)(int);

inline int foo1(int x) {
    return x * x;
}

inline int foo2(int x) {
    return x + x;
}

f bar1(void) {
    return foo1;
}

f bar2(void) {
    return foo2;
}

int main() {
    return 0;
}
