#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <type_traits>
#include <utility>

using t7 = std::sorted_equivalent_t;

std::sorted_equivalent_t f15() { return std::sorted_equivalent_t(); }

std::sorted_equivalent_t f16(const std::sorted_equivalent_t &a0) {
  return std::sorted_equivalent_t(a0);
}

std::sorted_equivalent_t f17(std::sorted_equivalent_t &&a0) {
  return std::sorted_equivalent_t(std::move(a0));
}

std::sorted_equivalent_t &f18(std::sorted_equivalent_t &a0,
                              const std::sorted_equivalent_t &a1) {
  return a0.operator=(a1);
}

std::sorted_equivalent_t &f19(std::sorted_equivalent_t &a0,
                              std::sorted_equivalent_t &&a1) {
  return a0.operator=(std::move(a1));
}

using t8 = std::sorted_unique_t;

std::sorted_unique_t f20() { return std::sorted_unique_t(); }

std::sorted_unique_t f21(const std::sorted_unique_t &a0) {
  return std::sorted_unique_t(a0);
}

std::sorted_unique_t f22(std::sorted_unique_t &&a0) {
  return std::sorted_unique_t(std::move(a0));
}

std::sorted_unique_t &f23(std::sorted_unique_t &a0,
                          const std::sorted_unique_t &a1) {
  return a0.operator=(a1);
}

std::sorted_unique_t &f24(std::sorted_unique_t &a0, std::sorted_unique_t &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> std::underlying_type_t<T1> f25(T1 a0) {
  return std::to_underlying<T1>(a0);
}

void f26() { return std::unreachable(); }
