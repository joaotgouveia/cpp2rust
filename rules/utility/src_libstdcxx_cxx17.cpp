#include <algorithm>
#include <utility>

template <typename T1> std::add_const_t<T1> &f8(T1 &a0) {
  return std::as_const<T1>(a0);
}

using t7 = std::in_place_t;

std::in_place_t f9() { return std::in_place_t(); }

std::in_place_t f10(const std::in_place_t &a0) { return std::in_place_t(a0); }

std::in_place_t f11(std::in_place_t &&a0) {
  return std::in_place_t(std::move(a0));
}

std::in_place_t &f12(std::in_place_t &a0, const std::in_place_t &a1) {
  return a0.operator=(a1);
}

std::in_place_t &f13(std::in_place_t &a0, std::in_place_t &&a1) {
  return a0.operator=(std::move(a1));
}
