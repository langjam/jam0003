module dbasm.tokens;

enum tokens
{
    load,        // load
    add,         // add
    substract,   // sub
    increment,   // inc
    decrement,   // dec
    times,       // times
    division,    // div
    shiftLeft,   // left
    shiftRight,  // right
    or,          // or
    and,         // and
    compare,     // comp
    same,        // same
    not,         // not
    more,        // more
    less,        // less
    moreOrSame,  // smore
    lessOrSame,  // sless
    whileLoop,   // while
    loop,        // loop
    jump,        // jump
    windowSize,  // screen
    setRegister, // set
    data         // data
}
