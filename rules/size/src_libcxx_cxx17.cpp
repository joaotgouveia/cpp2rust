#include <cstddef>
#include <iterator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> auto f1(const T1 &a0) { return std::size<T1>(a0); }

template <typename T1, std::size_t T2> std::size_t f2(const T1 (&a0)[T2]) {
  return std::size<T1, T2>(a0);
}
