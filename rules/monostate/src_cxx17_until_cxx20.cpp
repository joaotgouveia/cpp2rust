#include <utility>
#include <variant>

bool f7(std::monostate a0, std::monostate a1) {
  return std::move(a0) != std::move(a1);
}

bool f8(std::monostate a0, std::monostate a1) {
  return std::move(a0) < std::move(a1);
}

bool f9(std::monostate a0, std::monostate a1) {
  return std::move(a0) <= std::move(a1);
}

bool f10(std::monostate a0, std::monostate a1) {
  return std::move(a0) > std::move(a1);
}

bool f11(std::monostate a0, std::monostate a1) {
  return std::move(a0) >= std::move(a1);
}
