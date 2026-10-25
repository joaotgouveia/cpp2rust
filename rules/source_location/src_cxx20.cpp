#include <source_location>
#include <utility>

using t1 = std::source_location;

consteval std::source_location f1() { return std::source_location::current(); }

std::source_location f2() { return std::source_location(); }

const char *f3(const std::source_location &a0) { return a0.file_name(); }

const char *f4(const std::source_location &a0) { return a0.function_name(); }

std::source_location f5(const std::source_location &a0) {
  return std::source_location(a0);
}

std::source_location f6(std::source_location &&a0) {
  return std::source_location(std::move(a0));
}

std::source_location &f7(std::source_location &a0,
                         const std::source_location &a1) {
  return a0.operator=(a1);
}

std::source_location &f8(std::source_location &a0, std::source_location &&a1) {
  return a0.operator=(std::move(a1));
}
