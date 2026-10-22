#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <barrier>
#include <utility>

template <typename T1> std::unique_ptr<T1> f54() {
  return std::make_unique_for_overwrite<T1>();
}

template <typename T1> auto f55(std::size_t a0) {
  return std::make_unique_for_overwrite<T1>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_ostream<T1, T2> &f56(std::basic_ostream<T1, T2> &a0,
                                const std::unique_ptr<T3, T4> &a1) {
  return a0 << a1;
}
