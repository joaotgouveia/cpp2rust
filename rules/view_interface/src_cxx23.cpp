#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1> auto f15(std::ranges::view_interface<T1> &a0) {
  return a0.cbegin();
}

template <typename T1> auto f16(const std::ranges::view_interface<T1> &a0) {
  return a0.cbegin();
}

template <typename T1> auto f17(std::ranges::view_interface<T1> &a0) {
  return a0.cend();
}

template <typename T1> auto f18(const std::ranges::view_interface<T1> &a0) {
  return a0.cend();
}
