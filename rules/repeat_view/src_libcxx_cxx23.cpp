#include <iterator>
#include <ranges>
#include <tuple>
#include <utility>

template <typename T1, typename T2>
std::ranges::repeat_view<T1, T2> f32()
  requires(std::default_initializable<T1>) &&
          ((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return std::ranges::repeat_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::repeat_view<T1, T2> f33(const T1 &a0, T2 a1)
  requires(std::copy_constructible<T1>) &&
          ((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return std::ranges::repeat_view<T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::ranges::repeat_view<T1, T2> f34(T1 &&a0, T2 a1)
  requires((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return std::ranges::repeat_view<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
auto f35(const std::ranges::repeat_view<T1, T2> &a0)
  requires((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f36(const std::ranges::repeat_view<T1, T2> &a0)
  requires((!std::same_as<T2, std::unreachable_sentinel_t>)) &&
          ((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return a0.end();
}

template <typename T1, typename T2>
std::unreachable_sentinel_t f37(const std::ranges::repeat_view<T1, T2> &a0)
  requires((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return a0.end();
}

template <typename T1, typename T2>
auto f38(const std::ranges::repeat_view<T1, T2> &a0)
  requires((!std::same_as<T2, std::unreachable_sentinel_t>)) &&
          ((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return a0.size();
}

template <typename T1, typename T2, typename... T3, typename... T4>
std::ranges::repeat_view<T1, T2> f39(std::piecewise_construct_t a0,
                                     std::tuple<T3...> a1, std::tuple<T4...> a2)
  requires((std::constructible_from<T1, T3...> &&
            std::constructible_from<T2, T4...>)) &&
          ((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return std::ranges::repeat_view<T1, T2>(std::move(a0), std::move(a1),
                                          std::move(a2));
}

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
bool f40(std::ranges::repeat_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::repeat_view<T1, T2>> ||
           std::ranges::forward_range<std::ranges::repeat_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          ((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
bool f41(const std::ranges::repeat_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::repeat_view<T1, T2> const> ||
           std::ranges::forward_range<
               std::ranges::repeat_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          ((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
bool f42(std::ranges::repeat_view<T1, T2> &a0)
  requires((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
bool f43(const std::ranges::repeat_view<T1, T2> &a0)
  requires((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
auto f44(std::ranges::repeat_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::repeat_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          ((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
auto f45(const std::ranges::repeat_view<T1, T2> &a0)
  requires(std::ranges::range<std::ranges::repeat_view<T1, T2> const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::repeat_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          ((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
decltype(auto) f46(std::ranges::repeat_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::repeat_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          ((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
decltype(auto) f47(const std::ranges::repeat_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
              std::ranges::repeat_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          ((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
decltype(auto) f48(std::ranges::repeat_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::repeat_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::repeat_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          ((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return a0.back();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
decltype(auto) f49(const std::ranges::repeat_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::repeat_view<T1, T2> const> &&
           std::ranges::common_range<std::ranges::repeat_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          ((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return a0.back();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f50(std::ranges::repeat_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          ((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return a0.template operator[]<T3>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
decltype(auto) f51(const std::ranges::repeat_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          ((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return a0.template operator[]<T3>(std::move(a1));
}

template <typename T1, typename T2>
using t6 =
    typename std::ranges::repeat_view<T1, T2>::__iterator::iterator_concept;

template <typename T1, typename T2>
using t7 =
    typename std::ranges::repeat_view<T1, T2>::__iterator::iterator_category;

template <typename T1, typename T2>
using t8 = typename std::ranges::repeat_view<T1, T2>::__iterator::value_type;

template <typename T1, typename T2>
using t9 =
    typename std::ranges::repeat_view<T1, T2>::__iterator::difference_type;

template <typename T1, typename T2>
typename std::ranges::repeat_view<T1, T2>::__iterator f52() {
  return typename std::ranges::repeat_view<T1, T2>::__iterator();
}

template <typename T1, typename T2>
const T1 &f53(const typename std::ranges::repeat_view<T1, T2>::__iterator &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
const T1 &
f54(const typename std::ranges::repeat_view<T1, T2>::__iterator &a0,
    typename std::ranges::repeat_view<T1, T2>::__iterator::difference_type a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2>
auto f55(const typename std::ranges::repeat_view<T1, T2>::__iterator &a0,
         const typename std::ranges::repeat_view<T1, T2>::__iterator &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, typename T2>
bool f56(const typename std::ranges::repeat_view<T1, T2>::__iterator &a0,
         const typename std::ranges::repeat_view<T1, T2>::__iterator &a1) {
  return operator==(a0, a1);
}
