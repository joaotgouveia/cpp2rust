#include <utility>
#include <variant>

using t1 = std::monostate;

std::monostate f1() { return std::monostate(); }

std::monostate f2(const std::monostate &a0) { return std::monostate(a0); }

std::monostate f3(std::monostate &&a0) { return std::monostate(std::move(a0)); }

std::monostate &f4(std::monostate &a0, const std::monostate &a1) {
  return a0.operator=(a1);
}

std::monostate &f5(std::monostate &a0, std::monostate &&a1) {
  return a0.operator=(std::move(a1));
}

bool f6(std::monostate a0, std::monostate a1) {
  return std::move(a0) == std::move(a1);
}
