#include <compare>
#include <format>
#include <utility>
#include <variant>

std::strong_ordering f12(std::monostate a0, std::monostate a1) {
  return std::move(a0) <=> std::move(a1);
}
