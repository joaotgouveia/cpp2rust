#include <ranges>
#include <utility>

template <typename T1, typename T2>
std::ranges::take_while_view<T1, T2> f34()
  requires(std::default_initializable<T1> && std::default_initializable<T2>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return std::ranges::take_while_view<T1, T2>();
}

template <typename T1, typename T2>
auto f35(std::ranges::take_while_view<T1, T2> &a0)
  requires((!std::ranges::__simple_view<T1>)) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f36(std::ranges::take_while_view<T1, T2> &a0)
  requires((!std::ranges::__simple_view<T1>)) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.end();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
bool f37(std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::take_while_view<T1, T2>> ||
           std::ranges::forward_range<std::ranges::take_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::take_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
bool f38(const std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::sized_range<
               std::ranges::take_while_view<T1, T2> const> ||
           std::ranges::forward_range<
               std::ranges::take_while_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::take_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
bool f39(std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
bool f40(const std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
auto f41(std::ranges::take_while_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::take_while_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::take_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
auto f42(const std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::range<std::ranges::take_while_view<T1, T2> const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::take_while_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::take_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
auto f43(std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::take_while_view<T1, T2>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<std::ranges::take_while_view<T1, T2>>,
               std::ranges::iterator_t<
                   std::ranges::take_while_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::take_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
auto f44(const std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
               std::ranges::take_while_view<T1, T2> const> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<
                   std::ranges::take_while_view<T1, T2> const>,
               std::ranges::iterator_t<
                   std::ranges::take_while_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::take_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
decltype(auto) f45(std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::take_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::take_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
decltype(auto) f46(const std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
              std::ranges::take_while_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::take_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
decltype(auto) f47(std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::take_while_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::take_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::take_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.back();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
decltype(auto) f48(const std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::take_while_view<T1, T2> const> &&
           std::ranges::common_range<
               std::ranges::take_while_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::take_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.back();
}

template <typename T1, typename T2, bool T3>
using t3 =
    typename std::ranges::take_while_view<T1, T2>::template __sentinel<T3>;

template <typename T1, typename T2, bool T3>
typename std::ranges::take_while_view<T1, T2>::template __sentinel<T3> f49() {
  return
      typename std::ranges::take_while_view<T1, T2>::template __sentinel<T3>();
}

template <typename T1, typename T2, bool T3>
typename std::ranges::take_while_view<T1, T2>::template __sentinel<T3>
f50(std::ranges::sentinel_t<typename std::conditional<T3, const T1, T1>::type>
        a0,
    const T2 *a1) {
  return typename std::ranges::take_while_view<T1, T2>::template __sentinel<T3>(
      std::forward<std::ranges::sentinel_t<
          typename std::conditional<T3, const T1, T1>::type>>(a0),
      a1);
}

template <typename T1, typename T2, bool T3>
typename std::ranges::take_while_view<T1, T2>::template __sentinel<T3>
f51(typename std::ranges::take_while_view<T1, T2>::template __sentinel<!T3> a0)
  requires(T3 &&
           std::convertible_to<
               std::ranges::sentinel_t<T1>,
               std::ranges::sentinel_t<typename std::ranges::take_while_view<
                   T1, T2>::template __sentinel<T3>::_Base>>)
{
  return typename std::ranges::take_while_view<T1, T2>::template __sentinel<T3>(
      std::forward<typename std::ranges::take_while_view<
          T1, T2>::template __sentinel<!T3>>(a0));
}

template <typename T1, typename T2, bool T3>
std::ranges::sentinel_t<typename std::conditional<T3, const T1, T1>::type>
f52(const typename std::ranges::take_while_view<T1, T2>::template __sentinel<T3>
        &a0) {
  return a0.base();
}

template <typename T1, typename T2, bool T3, bool T4>
bool f53(
    const std::ranges::iterator_t<
        typename std::conditional<T4, const T1, T1>::type> &a0,
    const typename std::ranges::take_while_view<T1, T2>::template __sentinel<T3>
        &a1)
  requires(std::sentinel_for<
           std::ranges::sentinel_t<typename std::ranges::take_while_view<
               T1, T2>::template __sentinel<T3>::_Base>,
           std::ranges::iterator_t<std::__maybe_const<T4, T1>>>)
{
  return operator==(a0, a1);
}

template <typename T1, typename T2, bool T3>
bool f54(
    const std::ranges::iterator_t<
        typename std::conditional<T3, const T1, T1>::type> &a0,
    const typename std::ranges::take_while_view<T1, T2>::template __sentinel<T3>
        &a1) {
  return operator==(a0, a1);
}
