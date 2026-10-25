#include <algorithm>
#include <utility>

using t8 = std::sorted_equivalent_t;

std::sorted_equivalent_t f21() { return std::sorted_equivalent_t(); }

std::sorted_equivalent_t f22(const std::sorted_equivalent_t &a0) {
  return std::sorted_equivalent_t(a0);
}

std::sorted_equivalent_t f23(std::sorted_equivalent_t &&a0) {
  return std::sorted_equivalent_t(std::move(a0));
}

std::sorted_equivalent_t &f24(std::sorted_equivalent_t &a0,
                              const std::sorted_equivalent_t &a1) {
  return a0.operator=(a1);
}

std::sorted_equivalent_t &f25(std::sorted_equivalent_t &a0,
                              std::sorted_equivalent_t &&a1) {
  return a0.operator=(std::move(a1));
}

using t9 = std::sorted_unique_t;

std::sorted_unique_t f26() { return std::sorted_unique_t(); }

std::sorted_unique_t f27(const std::sorted_unique_t &a0) {
  return std::sorted_unique_t(a0);
}

std::sorted_unique_t f28(std::sorted_unique_t &&a0) {
  return std::sorted_unique_t(std::move(a0));
}

std::sorted_unique_t &f29(std::sorted_unique_t &a0,
                          const std::sorted_unique_t &a1) {
  return a0.operator=(a1);
}

std::sorted_unique_t &f30(std::sorted_unique_t &a0, std::sorted_unique_t &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> std::underlying_type_t<T1> f31(T1 a0) {
  return std::to_underlying<T1>(std::move(a0));
}

void f32() { return std::unreachable(); }
