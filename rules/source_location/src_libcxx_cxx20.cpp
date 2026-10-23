#include <source_location>
#include <utility>

std::uint_least32_t f10(const std::source_location &a0) { return a0.line(); }

std::uint_least32_t f11(const std::source_location &a0) { return a0.column(); }
