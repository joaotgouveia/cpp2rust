#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <ios>
#include <iosfwd>
#include <utility>

template <typename T1, typename T2>
using t12 = typename std::istreambuf_iterator<T1, T2>::pointer;

template <typename T1, typename T2>
std::istreambuf_iterator<T1, T2> f16(std::default_sentinel_t a0) {
  return std::istreambuf_iterator<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
bool f17(const std::istreambuf_iterator<T1, T2> &a0,
         std::default_sentinel_t a1) {
  return a0 == std::move(a1);
}
