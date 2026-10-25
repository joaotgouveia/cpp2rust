#include <algorithm>
#include <array>
#include <utility>

template <typename T1> auto f7(const T1 &a0) { return std::cbegin<T1>(a0); }

template <typename T1> auto f8(const T1 &a0) { return std::cend<T1>(a0); }

template <typename T1> auto f9(const T1 &a0) { return std::crbegin<T1>(a0); }

template <typename T1> auto f10(const T1 &a0) { return std::crend<T1>(a0); }

template <typename T1> auto f11(const T1 &a0) { return std::rbegin<T1>(a0); }

template <typename T1> auto f12(T1 &a0) { return std::rbegin<T1>(a0); }

template <typename T1>
std::reverse_iterator<const T1 *> f13(std::initializer_list<T1> a0) {
  return std::rbegin<T1>(std::forward<std::initializer_list<T1>>(a0));
}

template <typename T1, std::size_t T2>
std::reverse_iterator<T1 *> f14(T1 (&a0)[T2]) {
  return std::rbegin<T1, T2>(a0);
}

template <typename T1> auto f15(const T1 &a0) { return std::rend<T1>(a0); }

template <typename T1> auto f16(T1 &a0) { return std::rend<T1>(a0); }

template <typename T1>
std::reverse_iterator<const T1 *> f17(std::initializer_list<T1> a0) {
  return std::rend<T1>(std::forward<std::initializer_list<T1>>(a0));
}

template <typename T1, std::size_t T2>
std::reverse_iterator<T1 *> f18(T1 (&a0)[T2]) {
  return std::rend<T1, T2>(a0);
}
