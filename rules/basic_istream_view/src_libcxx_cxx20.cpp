#include <istream>
#include <iterator>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3>
std::ranges::basic_istream_view<T1, T2, T3> f24(std::basic_istream<T2, T3> &a0)
  requires(std::default_initializable<T1> &&
           std::ranges::__stream_extractable<T1, T2, T3>)
{
  return std::ranges::basic_istream_view<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
auto f25(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::default_initializable<T1> &&
           std::ranges::__stream_extractable<T1, T2, T3>)
{
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
std::default_sentinel_t
f26(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::default_initializable<T1> &&
           std::ranges::__stream_extractable<T1, T2, T3>)
{
  return a0.end();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
bool f27(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::sized_range<
               std::ranges::basic_istream_view<T1, T2, T3>> ||
           std::ranges::forward_range<
               std::ranges::basic_istream_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<
               std::ranges::basic_istream_view<T1, T2, T3>,
               std::remove_cv_t<std::ranges::basic_istream_view<T1, T2, T3>>>)
{
  return a0.empty();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
bool f28(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::sized_range<
               std::ranges::basic_istream_view<T1, T2, T3> const> ||
           std::ranges::forward_range<
               std::ranges::basic_istream_view<T1, T2, T3> const>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<
               std::ranges::basic_istream_view<T1, T2, T3>,
               std::remove_cv_t<std::ranges::basic_istream_view<T1, T2, T3>>>)
{
  return a0.empty();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
bool f29(std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
bool f30(const std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
auto f31(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<
               std::ranges::basic_istream_view<T1, T2, T3>,
               std::remove_cv_t<std::ranges::basic_istream_view<T1, T2, T3>>>)
{
  return a0.data();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
auto f32(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::range<
               std::ranges::basic_istream_view<T1, T2, T3> const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::basic_istream_view<T1, T2, T3> const>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<
               std::ranges::basic_istream_view<T1, T2, T3>,
               std::remove_cv_t<std::ranges::basic_istream_view<T1, T2, T3>>>)
{
  return a0.data();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
auto f33(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<
               std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<
                   std::ranges::basic_istream_view<T1, T2, T3>>,
               std::ranges::iterator_t<
                   std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<
               std::ranges::basic_istream_view<T1, T2, T3>,
               std::remove_cv_t<std::ranges::basic_istream_view<T1, T2, T3>>>)
{
  return a0.size();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
auto f34(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<
               std::ranges::basic_istream_view<T1, T2, T3> const> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<
                   std::ranges::basic_istream_view<T1, T2, T3> const>,
               std::ranges::iterator_t<
                   std::ranges::basic_istream_view<T1, T2, T3> const>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<
               std::ranges::basic_istream_view<T1, T2, T3>,
               std::remove_cv_t<std::ranges::basic_istream_view<T1, T2, T3>>>)
{
  return a0.size();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
decltype(auto) f35(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<
              std::ranges::basic_istream_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<
               std::ranges::basic_istream_view<T1, T2, T3>,
               std::remove_cv_t<std::ranges::basic_istream_view<T1, T2, T3>>>)
{
  return a0.front();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
decltype(auto) f36(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<
              std::ranges::basic_istream_view<T1, T2, T3> const>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<
               std::ranges::basic_istream_view<T1, T2, T3>,
               std::remove_cv_t<std::ranges::basic_istream_view<T1, T2, T3>>>)
{
  return a0.front();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
decltype(auto) f37(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::ranges::common_range<
               std::ranges::basic_istream_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<
               std::ranges::basic_istream_view<T1, T2, T3>,
               std::remove_cv_t<std::ranges::basic_istream_view<T1, T2, T3>>>)
{
  return a0.back();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
decltype(auto) f38(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::basic_istream_view<T1, T2, T3> const> &&
           std::ranges::common_range<
               std::ranges::basic_istream_view<T1, T2, T3> const>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<
               std::ranges::basic_istream_view<T1, T2, T3>,
               std::remove_cv_t<std::ranges::basic_istream_view<T1, T2, T3>>>)
{
  return a0.back();
}
