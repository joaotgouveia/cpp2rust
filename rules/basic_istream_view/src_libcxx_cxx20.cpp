#include <istream>
#include <iterator>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
bool f22(std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
bool f23(const std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
bool f24(std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
bool f25(const std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
auto f26(std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
auto f27(const std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
auto f28(std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
auto f29(const std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
decltype(auto) f30(std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
decltype(auto) f31(const std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
decltype(auto) f32(std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
decltype(auto) f33(const std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.back();
}
