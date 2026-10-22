#include "../common/init.h"
#include "../common/traits.h"
#include <initializer_list>
#include <iterator>
#include <utility>

template <typename T1> auto f1(const T1 &a0) { return std::data<T1>(a0); }

template <typename T1> auto f2(T1 &a0) { return std::data<T1>(a0); }

template <typename T1> const T1 *f3(std::initializer_list<T1> a0) {
  return std::data<T1>(a0);
}

template <typename T1, std::size_t T2> T1 *f4(T1 (&a0)[T2]) {
  return std::data<T1, T2>(a0);
}
