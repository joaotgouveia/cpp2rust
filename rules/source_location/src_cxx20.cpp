#include "../common/init.h"
#include "../common/traits.h"
#include <source_location>
#include <utility>

using t1 = std::source_location;

std::source_location f1() { return std::source_location(); }

unsigned int f2(const std::source_location &a0) { return a0.line(); }

unsigned int f3(const std::source_location &a0) { return a0.column(); }

const char *f4(const std::source_location &a0) { return a0.file_name(); }

const char *f5(const std::source_location &a0) { return a0.function_name(); }

std::source_location f6(const std::source_location &a0) {
  return std::source_location(a0);
}

std::source_location f7(std::source_location &&a0) {
  return std::source_location(std::move(a0));
}

std::source_location &f8(std::source_location &a0,
                         const std::source_location &a1) {
  return a0.operator=(a1);
}

std::source_location &f9(std::source_location &a0, std::source_location &&a1) {
  return a0.operator=(std::move(a1));
}
