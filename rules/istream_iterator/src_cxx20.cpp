#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <iterator>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
std::istream_iterator<T1, T2, T3, T4> f13(std::default_sentinel_t a0) {
  return std::istream_iterator<T1, T2, T3, T4>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
bool f14(const std::istream_iterator<T1, T2, T3, T4> &a0,
         std::default_sentinel_t a1) {
  return a0 == std::move(a1);
}
