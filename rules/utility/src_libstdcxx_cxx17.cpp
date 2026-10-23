#include <algorithm>
#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> std::add_const_t<T1> &f2(T1 &a0) {
  return std::as_const<T1>(a0);
}

using t6 = std::in_place_t;

std::in_place_t f3() { return std::in_place_t(); }

std::in_place_t f4(const std::in_place_t &a0) { return std::in_place_t(a0); }

std::in_place_t f5(std::in_place_t &&a0) {
  return std::in_place_t(std::move(a0));
}

std::in_place_t &f6(std::in_place_t &a0, const std::in_place_t &a1) {
  return a0.operator=(a1);
}

std::in_place_t &f7(std::in_place_t &a0, std::in_place_t &&a1) {
  return a0.operator=(std::move(a1));
}
