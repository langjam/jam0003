#include <source_location>

[[noreturn]] static void unreachable(
    const std::source_location location = std::source_location::current()) {
#ifndef NDEBUG  // debug mode
    std::cerr << "unreachable statement reached from " < < < < std::endl;
    return;
#endif
}
