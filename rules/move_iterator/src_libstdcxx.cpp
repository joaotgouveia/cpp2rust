#include <algorithm>
#include <utility>

template <typename T1> std::move_iterator<T1> f25() {
  return std::move_iterator<T1>();
}

template <typename T1>
std::move_iterator<T1> f26(std::move_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}
