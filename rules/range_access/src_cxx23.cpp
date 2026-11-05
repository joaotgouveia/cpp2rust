#include <algorithm>
#include <array>
#include <initializer_list>
#include <utility>

template <typename T1> auto f1(const T1 &a0) { return std::begin(a0); }

template <typename T1> auto f2(T1 &a0) { return std::begin(a0); }

template <typename T1, std::size_t T2> T1 *f3(T1 (&a0)[T2]) {
  return std::begin(a0);
}

template <typename T1> auto f4(const T1 &a0) { return std::cbegin(a0); }

template <typename T1> auto f5(const T1 &a0) { return std::cend(a0); }

template <typename T1> auto f6(const T1 &a0) { return std::crbegin(a0); }

template <typename T1> auto f7(const T1 &a0) { return std::crend(a0); }

template <typename T1> auto f8(const T1 &a0) { return std::data(a0); }

template <typename T1> auto f9(T1 &a0) { return std::data(a0); }

template <typename T1> const T1 *f10(std::initializer_list<T1> a0) {
  return std::data(std::forward<std::initializer_list<T1>>(a0));
}

template <typename T1, std::size_t T2> T1 *f11(T1 (&a0)[T2]) {
  return std::data(a0);
}

template <typename T1> auto f12(const T1 &a0) { return std::empty(a0); }

template <typename T1> bool f13(std::initializer_list<T1> a0) {
  return std::empty(std::forward<std::initializer_list<T1>>(a0));
}

template <typename T1, std::size_t T2> bool f14(const T1 (&a0)[T2]) {
  return std::empty(a0);
}

template <typename T1> auto f15(const T1 &a0) { return std::end(a0); }

template <typename T1> auto f16(T1 &a0) { return std::end(a0); }

template <typename T1, std::size_t T2> T1 *f17(T1 (&a0)[T2]) {
  return std::end(a0);
}

template <typename T1> auto f18(const T1 &a0) { return std::rbegin(a0); }

template <typename T1> auto f19(T1 &a0) { return std::rbegin(a0); }

template <typename T1>
std::reverse_iterator<const T1 *> f20(std::initializer_list<T1> a0) {
  return std::rbegin(std::forward<std::initializer_list<T1>>(a0));
}

template <typename T1, std::size_t T2>
std::reverse_iterator<T1 *> f21(T1 (&a0)[T2]) {
  return std::rbegin(a0);
}

template <typename T1> auto f22(const T1 &a0) { return std::rend(a0); }

template <typename T1> auto f23(T1 &a0) { return std::rend(a0); }

template <typename T1>
std::reverse_iterator<const T1 *> f24(std::initializer_list<T1> a0) {
  return std::rend(std::forward<std::initializer_list<T1>>(a0));
}

template <typename T1, std::size_t T2>
std::reverse_iterator<T1 *> f25(T1 (&a0)[T2]) {
  return std::rend(a0);
}

template <typename T1> auto f26(const T1 &a0) { return std::size(a0); }

template <typename T1, std::size_t T2> std::size_t f27(const T1 (&a0)[T2]) {
  return std::size(a0);
}

template <typename T1> auto f28(const T1 &a0) { return std::ssize(a0); }

template <typename T1, std::ptrdiff_t T2>
std::ptrdiff_t f29(const T1 (&a0)[T2]) {
  return std::ssize(a0);
}
