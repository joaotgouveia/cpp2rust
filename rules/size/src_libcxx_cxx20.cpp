#include "../common/init.h"
#include "../common/traits.h"
#include <cstddef>
#include <iterator>
#include <utility>

template <typename T1> auto f3(const T1 &a0) { return std::ssize<T1>(a0); }

template <typename T1, std::ptrdiff_t T2>
std::ptrdiff_t f4(const T1 (&a0)[T2]) {
  return std::ssize<T1, T2>(a0);
}
