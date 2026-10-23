#include <initializer_list>
#include <iterator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> auto f1(const T1 &a0) { return std::crbegin<T1>(a0); }

template <typename T1> auto f2(const T1 &a0) { return std::crend<T1>(a0); }

template <typename T1> auto f3(const T1 &a0) { return std::rbegin<T1>(a0); }

template <typename T1> auto f4(T1 &a0) { return std::rbegin<T1>(a0); }

template <typename T1>
std::reverse_iterator<const T1 *> f5(std::initializer_list<T1> a0) {
  return std::rbegin<T1>(a0);
}

template <typename T1, std::size_t T2>
std::reverse_iterator<T1 *> f6(T1 (&a0)[T2]) {
  return std::rbegin<T1, T2>(a0);
}

template <typename T1> auto f7(const T1 &a0) { return std::rend<T1>(a0); }

template <typename T1> auto f8(T1 &a0) { return std::rend<T1>(a0); }

template <typename T1>
std::reverse_iterator<const T1 *> f9(std::initializer_list<T1> a0) {
  return std::rend<T1>(a0);
}

template <typename T1, std::size_t T2>
std::reverse_iterator<T1 *> f10(T1 (&a0)[T2]) {
  return std::rend<T1, T2>(a0);
}
