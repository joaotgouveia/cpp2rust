#include <initializer_list>
#include <iterator>
#include <utility>

template <typename T1> auto f1(const T1 &a0) { return std::empty<T1>(a0); }

template <typename T1> bool f2(std::initializer_list<T1> a0) {
  return std::empty<T1>(std::move(a0));
}

template <typename T1, std::size_t T2> bool f3(const T1 (&a0)[T2]) {
  return std::empty<T1, T2>(a0);
}
