#include <istream>
#include <iterator>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3>
std::ranges::basic_istream_view<T1, T2, T3> f30(std::basic_istream<T2, T3> &a0)
  requires(std::default_initializable<T1> &&
           std::ranges::__stream_extractable<T1, T2, T3>)
{
  return std::ranges::basic_istream_view<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
auto f31(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::default_initializable<T1> &&
           std::ranges::__stream_extractable<T1, T2, T3>)
{
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
std::default_sentinel_t
f32(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::default_initializable<T1> &&
           std::ranges::__stream_extractable<T1, T2, T3>)
{
  return a0.end();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
bool f33(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::sized_range<
               std::ranges::basic_istream_view<T1, T2, T3>> ||
           std::ranges::forward_range<
               std::ranges::basic_istream_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__stream_extractable<T1, T2, T3>)
{
  return a0.empty();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
bool f34(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::sized_range<
               std::ranges::basic_istream_view<T1, T2, T3> const> ||
           std::ranges::forward_range<
               std::ranges::basic_istream_view<T1, T2, T3> const>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__stream_extractable<T1, T2, T3>)
{
  return a0.empty();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
bool f35(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::default_initializable<T1> &&
           std::ranges::__stream_extractable<T1, T2, T3>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
bool f36(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::default_initializable<T1> &&
           std::ranges::__stream_extractable<T1, T2, T3>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
auto f37(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__stream_extractable<T1, T2, T3>)
{
  return a0.data();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
auto f38(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::range<
               std::ranges::basic_istream_view<T1, T2, T3> const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::basic_istream_view<T1, T2, T3> const>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__stream_extractable<T1, T2, T3>)
{
  return a0.data();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
auto f39(std::ranges::basic_istream_view<T1, T2, T3> &a0)
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
           std::ranges::__stream_extractable<T1, T2, T3>)
{
  return a0.size();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
auto f40(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<
               std::ranges::basic_istream_view<T1, T2, T3> const> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<
                   std::ranges::basic_istream_view<T1, T2, T3> const>,
               std::ranges::iterator_t<
                   std::ranges::basic_istream_view<T1, T2, T3> const>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__stream_extractable<T1, T2, T3>)
{
  return a0.size();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
decltype(auto) f41(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<
              std::ranges::basic_istream_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__stream_extractable<T1, T2, T3>)
{
  return a0.front();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
decltype(auto) f42(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<
              std::ranges::basic_istream_view<T1, T2, T3> const>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__stream_extractable<T1, T2, T3>)
{
  return a0.front();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
decltype(auto) f43(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::ranges::common_range<
               std::ranges::basic_istream_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__stream_extractable<T1, T2, T3>)
{
  return a0.back();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
decltype(auto) f44(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::basic_istream_view<T1, T2, T3> const> &&
           std::ranges::common_range<
               std::ranges::basic_istream_view<T1, T2, T3> const>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__stream_extractable<T1, T2, T3>)
{
  return a0.back();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::ranges::basic_istream_view<T1, T2, T3>>
decltype(auto) f45(std::ranges::basic_istream_view<T1, T2, T3> &a0,
                   std::ranges::range_difference_t<T4> a1)
  requires(std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::ranges::random_access_range<T4>) &&
          (std::default_initializable<T1> &&
           std::ranges::__stream_extractable<T1, T2, T3>)
{
  return a0.template operator[]<T4>(
      std::forward<std::ranges::range_difference_t<T4>>(a1));
}

template <typename T1, typename T2, typename T3,
          typename T4 = const std::ranges::basic_istream_view<T1, T2, T3>>
decltype(auto) f46(const std::ranges::basic_istream_view<T1, T2, T3> &a0,
                   std::ranges::range_difference_t<T4> a1)
  requires(std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::ranges::random_access_range<T4>) &&
          (std::default_initializable<T1> &&
           std::ranges::__stream_extractable<T1, T2, T3>)
{
  return a0.template operator[]<T4>(
      std::forward<std::ranges::range_difference_t<T4>>(a1));
}

template <typename T1, typename T2, typename T3>
using t6 = typename std::ranges::basic_istream_view<T1, T2, T3>::__iterator;

template <typename T1, typename T2, typename T3>
using t7 =
    typename std::ranges::basic_istream_view<T1, T2,
                                             T3>::__iterator::iterator_concept;

template <typename T1, typename T2, typename T3>
using t8 =
    typename std::ranges::basic_istream_view<T1, T2,
                                             T3>::__iterator::difference_type;

template <typename T1, typename T2, typename T3>
using t9 = typename std::ranges::basic_istream_view<T1, T2,
                                                    T3>::__iterator::value_type;

template <typename T1, typename T2, typename T3>
typename std::ranges::basic_istream_view<T1, T2, T3>::__iterator
f47(std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return typename std::ranges::basic_istream_view<T1, T2, T3>::__iterator(a0);
}

template <typename T1, typename T2, typename T3>
typename std::ranges::basic_istream_view<T1, T2, T3>::__iterator
f48(typename std::ranges::basic_istream_view<T1, T2, T3>::__iterator &&a0) {
  return typename std::ranges::basic_istream_view<T1, T2, T3>::__iterator(
      std::move(a0));
}

template <typename T1, typename T2, typename T3>
typename std::ranges::basic_istream_view<T1, T2, T3>::__iterator &
f49(typename std::ranges::basic_istream_view<T1, T2, T3>::__iterator &a0,
    typename std::ranges::basic_istream_view<T1, T2, T3>::__iterator &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::ranges::basic_istream_view<T1, T2, T3>::__iterator &
f50(typename std::ranges::basic_istream_view<T1, T2, T3>::__iterator &a0) {
  return a0.operator++();
}

template <typename T1, typename T2, typename T3>
void f51(typename std::ranges::basic_istream_view<T1, T2, T3>::__iterator &a0,
         int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2, typename T3>
T1 &f52(const typename std::ranges::basic_istream_view<T1, T2, T3>::__iterator
            &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, typename T3>
bool f53(
    const typename std::ranges::basic_istream_view<T1, T2, T3>::__iterator &a0,
    std::default_sentinel_t a1) {
  return operator==(a0, std::move(a1));
}
