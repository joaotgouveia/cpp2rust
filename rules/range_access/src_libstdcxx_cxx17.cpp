#include <algorithm>
#include <array>
#include <initializer_list>
#include <utility>

template <typename T1> auto f19(const T1 &a0) { return std::data<T1>(a0); }

template <typename T1> auto f20(T1 &a0) { return std::data<T1>(a0); }

template <typename T1> const T1 *f21(std::initializer_list<T1> a0) {
  return std::data<T1>(a0);
}

template <typename T1, std::size_t T2> T1 *f22(T1 (&a0)[T2]) {
  return std::data<T1, T2>(a0);
}

template <typename T1> auto f23(const T1 &a0) { return std::empty<T1>(a0); }

template <typename T1> bool f24(std::initializer_list<T1> a0) {
  return std::empty<T1>(a0);
}

template <typename T1, std::size_t T2> bool f25(const T1 (&a0)[T2]) {
  return std::empty<T1, T2>(a0);
}

template <typename T1> auto f26(const T1 &a0) { return std::size<T1>(a0); }

template <typename T1, std::size_t T2> std::size_t f27(const T1 (&a0)[T2]) {
  return std::size<T1, T2>(a0);
}
