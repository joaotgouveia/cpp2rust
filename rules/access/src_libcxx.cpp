#include <iterator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> auto f1(const T1 &a0) -> decltype(std::begin<T1>(a0)) {
  return std::begin<T1>(a0);
}

template <typename T1> auto f2(T1 &a0) -> decltype(std::begin<T1>(a0)) {
  return std::begin<T1>(a0);
}

template <typename T1, std::size_t T2> T1 *f3(T1 (&a0)[T2]) {
  return std::begin<T1, T2>(a0);
}

template <typename T1> auto f4(const T1 &a0) -> decltype(std::end<T1>(a0)) {
  return std::end<T1>(a0);
}

template <typename T1> auto f5(T1 &a0) -> decltype(std::end<T1>(a0)) {
  return std::end<T1>(a0);
}

template <typename T1, std::size_t T2> T1 *f6(T1 (&a0)[T2]) {
  return std::end<T1, T2>(a0);
}
