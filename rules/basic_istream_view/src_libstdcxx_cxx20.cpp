#include <algorithm>
#include <istream>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3>
std::ranges::basic_istream_view<T1, T2, T3> f1(std::basic_istream<T2, T3> &a0)
  requires(std::default_initializable<T1> &&
           std::ranges::__detail::__stream_extractable<T1, T2, T3>)
{
  return std::ranges::basic_istream_view<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
auto f2(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::default_initializable<T1> &&
           std::ranges::__detail::__stream_extractable<T1, T2, T3>)
{
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
std::default_sentinel_t
f3(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::default_initializable<T1> &&
           std::ranges::__detail::__stream_extractable<T1, T2, T3>)
{
  return a0.end();
}

template <typename T1, typename T2, typename T3>
bool f4(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<
               std::ranges::basic_istream_view<T1, T2, T3>> &&
           (!std::ranges::sized_range<
               std::ranges::basic_istream_view<T1, T2, T3>>)) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__detail::__stream_extractable<T1, T2, T3>)
{
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
bool f5(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::sized_range<
              std::ranges::basic_istream_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__detail::__stream_extractable<T1, T2, T3>)
{
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
bool f6(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<
               const std::ranges::basic_istream_view<T1, T2, T3>> &&
           (!std::ranges::sized_range<
               const std::ranges::basic_istream_view<T1, T2, T3>>)) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__detail::__stream_extractable<T1, T2, T3>)
{
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
bool f7(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::sized_range<
              const std::ranges::basic_istream_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__detail::__stream_extractable<T1, T2, T3>)
{
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
bool f8(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::default_initializable<T1> &&
           std::ranges::__detail::__stream_extractable<T1, T2, T3>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3>
bool f9(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::default_initializable<T1> &&
           std::ranges::__detail::__stream_extractable<T1, T2, T3>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3>
auto f10(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__detail::__stream_extractable<T1, T2, T3>)
{
  return a0.data();
}

template <typename T1, typename T2, typename T3>
auto f11(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::range<
               const std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               const std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__detail::__stream_extractable<T1, T2, T3>)
{
  return a0.data();
}

template <typename T1, typename T2, typename T3>
auto f12(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<
               std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<
                   std::ranges::basic_istream_view<T1, T2, T3>>,
               std::ranges::iterator_t<
                   std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__detail::__stream_extractable<T1, T2, T3>)
{
  return a0.size();
}

template <typename T1, typename T2, typename T3>
auto f13(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<
               const std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<
                   const std::ranges::basic_istream_view<T1, T2, T3>>,
               std::ranges::iterator_t<
                   const std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__detail::__stream_extractable<T1, T2, T3>)
{
  return a0.size();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f14(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<
              std::ranges::basic_istream_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__detail::__stream_extractable<T1, T2, T3>)
{
  return a0.front();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f15(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<
              const std::ranges::basic_istream_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__detail::__stream_extractable<T1, T2, T3>)
{
  return a0.front();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f16(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::ranges::common_range<
               std::ranges::basic_istream_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__detail::__stream_extractable<T1, T2, T3>)
{
  return a0.back();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f17(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::ranges::common_range<
               const std::ranges::basic_istream_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__detail::__stream_extractable<T1, T2, T3>)
{
  return a0.back();
}

template <typename T1, typename T2, typename T3, typename T4>
decltype(auto) f18(std::ranges::basic_istream_view<T1, T2, T3> &a0,
                   std::ranges::range_difference_t<T4> a1)
  requires(std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::ranges::random_access_range<T4>) &&
          (std::default_initializable<T1> &&
           std::ranges::__detail::__stream_extractable<T1, T2, T3>)
{
  return a0.template operator[]<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
decltype(auto) f19(const std::ranges::basic_istream_view<T1, T2, T3> &a0,
                   std::ranges::range_difference_t<T4> a1)
  requires(std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::ranges::random_access_range<T4>) &&
          (std::default_initializable<T1> &&
           std::ranges::__detail::__stream_extractable<T1, T2, T3>)
{
  return a0.template operator[]<T4>(a1);
}
