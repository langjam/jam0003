#include "llvm_test.hh"

// this file is to force bazel to emit an object file
// so this library isn't "header-only"
int blah() { return 2; }
