#include <algorithm>
#include <istream>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3>
auto f18(std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
auto f19(const std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
auto f20(std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
auto f21(const std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.cend();
}
