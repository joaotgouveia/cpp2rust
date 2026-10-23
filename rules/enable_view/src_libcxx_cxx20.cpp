#include <ranges>
#include <utility>

using t1 = std::ranges::view_base;

std::ranges::view_base f1() { return std::ranges::view_base(); }

std::ranges::view_base f2(const std::ranges::view_base &a0) {
  return std::ranges::view_base(a0);
}

std::ranges::view_base f3(std::ranges::view_base &&a0) {
  return std::ranges::view_base(std::move(a0));
}

std::ranges::view_base &f4(std::ranges::view_base &a0,
                           const std::ranges::view_base &a1) {
  return a0.operator=(a1);
}

std::ranges::view_base &f5(std::ranges::view_base &a0,
                           std::ranges::view_base &&a1) {
  return a0.operator=(std::move(a1));
}
