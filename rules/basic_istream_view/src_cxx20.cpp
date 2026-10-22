#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <istream>
#include <iterator>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::ranges::basic_istream_view<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
std::ranges::basic_istream_view<T1, T2, T3> f1(std::basic_istream<T2, T3> &a0) {
  return std::ranges::basic_istream_view<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
auto f2(std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
std::default_sentinel_t
f3(const std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.end();
}
