#include <iterator>
#include <ranges>
#include <utility>

template <typename T1>
auto f32(std::ranges::take_view<T1> &a0)
  requires((!std::ranges::__simple_view<T1>))
{
  return a0.begin();
}

template <typename T1>
auto f33(std::ranges::take_view<T1> &a0)
  requires((!std::ranges::__simple_view<T1>))
{
  return a0.end();
}

template <typename T1, typename T2 = std::ranges::take_view<T1>>
bool f34(std::ranges::take_view<T1> &a0)
  requires(std::ranges::sized_range<std::ranges::take_view<T1>> ||
           std::ranges::forward_range<std::ranges::take_view<T1>>) &&
          (std::is_class_v<std::ranges::take_view<T1>> &&
           std::same_as<std::ranges::take_view<T1>,
                        std::remove_cv_t<std::ranges::take_view<T1>>>)
{
  return a0.empty();
}

template <typename T1, typename T2 = std::ranges::take_view<T1>>
bool f35(const std::ranges::take_view<T1> &a0)
  requires(std::ranges::sized_range<std::ranges::take_view<T1> const> ||
           std::ranges::forward_range<std::ranges::take_view<T1> const>) &&
          (std::is_class_v<std::ranges::take_view<T1>> &&
           std::same_as<std::ranges::take_view<T1>,
                        std::remove_cv_t<std::ranges::take_view<T1>>>)
{
  return a0.empty();
}

template <typename T1, typename T2 = std::ranges::take_view<T1>>
bool f36(std::ranges::take_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::take_view<T1>>
bool f37(const std::ranges::take_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::take_view<T1>>
auto f38(std::ranges::take_view<T1> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::take_view<T1>>>) &&
          (std::is_class_v<std::ranges::take_view<T1>> &&
           std::same_as<std::ranges::take_view<T1>,
                        std::remove_cv_t<std::ranges::take_view<T1>>>)
{
  return a0.data();
}

template <typename T1, typename T2 = std::ranges::take_view<T1>>
auto f39(const std::ranges::take_view<T1> &a0)
  requires(std::ranges::range<std::ranges::take_view<T1> const> &&
           std::contiguous_iterator<
               std::ranges::iterator_t<std::ranges::take_view<T1> const>>) &&
          (std::is_class_v<std::ranges::take_view<T1>> &&
           std::same_as<std::ranges::take_view<T1>,
                        std::remove_cv_t<std::ranges::take_view<T1>>>)
{
  return a0.data();
}

template <typename T1, typename T2 = std::ranges::take_view<T1>>
decltype(auto) f40(std::ranges::take_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::take_view<T1>>) &&
          (std::is_class_v<std::ranges::take_view<T1>> &&
           std::same_as<std::ranges::take_view<T1>,
                        std::remove_cv_t<std::ranges::take_view<T1>>>)
{
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::take_view<T1>>
decltype(auto) f41(const std::ranges::take_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::take_view<T1> const>) &&
          (std::is_class_v<std::ranges::take_view<T1>> &&
           std::same_as<std::ranges::take_view<T1>,
                        std::remove_cv_t<std::ranges::take_view<T1>>>)
{
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::take_view<T1>>
decltype(auto) f42(std::ranges::take_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::take_view<T1>> &&
           std::ranges::common_range<std::ranges::take_view<T1>>) &&
          (std::is_class_v<std::ranges::take_view<T1>> &&
           std::same_as<std::ranges::take_view<T1>,
                        std::remove_cv_t<std::ranges::take_view<T1>>>)
{
  return a0.back();
}

template <typename T1, typename T2 = std::ranges::take_view<T1>>
decltype(auto) f43(const std::ranges::take_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::take_view<T1> const> &&
           std::ranges::common_range<std::ranges::take_view<T1> const>) &&
          (std::is_class_v<std::ranges::take_view<T1>> &&
           std::same_as<std::ranges::take_view<T1>,
                        std::remove_cv_t<std::ranges::take_view<T1>>>)
{
  return a0.back();
}

template <typename T1, bool T2>
using t3 = typename std::ranges::take_view<T1>::template __sentinel<T2>;

template <typename T1, bool T2>
typename std::ranges::take_view<T1>::template __sentinel<T2> f44() {
  return typename std::ranges::take_view<T1>::template __sentinel<T2>();
}

template <typename T1, bool T2>
typename std::ranges::take_view<T1>::template __sentinel<T2>
f45(std::ranges::sentinel_t<typename std::conditional<T2, const T1, T1>::type>
        a0) {
  return typename std::ranges::take_view<T1>::template __sentinel<T2>(
      std::forward<std::ranges::sentinel_t<
          typename std::conditional<T2, const T1, T1>::type>>(a0));
}

template <typename T1, bool T2>
typename std::ranges::take_view<T1>::template __sentinel<T2>
f46(typename std::ranges::take_view<T1>::template __sentinel<!T2> a0)
  requires(T2 && std::convertible_to<
                     std::ranges::sentinel_t<T1>,
                     std::ranges::sentinel_t<typename std::ranges::take_view<
                         T1>::template __sentinel<T2>::_Base>>)
{
  return typename std::ranges::take_view<T1>::template __sentinel<T2>(
      std::forward<
          typename std::ranges::take_view<T1>::template __sentinel<!T2>>(a0));
}

template <typename T1, bool T2>
std::ranges::sentinel_t<typename std::conditional<T2, const T1, T1>::type>
f47(const typename std::ranges::take_view<T1>::template __sentinel<T2> &a0) {
  return a0.base();
}

template <typename T1, bool T2, bool T3>
bool f48(const std::counted_iterator<std::ranges::iterator_t<
             typename std::conditional<T3, const T1, T1>::type>> &a0,
         const typename std::ranges::take_view<T1>::template __sentinel<T2> &a1)
  requires(
      std::sentinel_for<std::ranges::sentinel_t<typename std::ranges::take_view<
                            T1>::template __sentinel<T2>::_Base>,
                        std::ranges::iterator_t<std::__maybe_const<T3, T1>>>)
{
  return operator==(a0, a1);
}

template <typename T1, bool T2>
bool f49(
    const std::counted_iterator<std::ranges::iterator_t<
        typename std::conditional<T2, const T1, T1>::type>> &a0,
    const typename std::ranges::take_view<T1>::template __sentinel<T2> &a1) {
  return operator==(a0, a1);
}
