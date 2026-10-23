#include <source_location>
#include <utility>

unsigned int f8(const std::source_location &a0) { return a0.line(); }

unsigned int f9(const std::source_location &a0) { return a0.column(); }
