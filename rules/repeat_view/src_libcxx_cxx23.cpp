#include <iterator>
#include <ranges>
#include <tuple>
#include <utility>

template <typename T1, typename T2>
std::ranges::repeat_view<T1, T2> f27()
  requires(std::default_initializable<T1>) &&
          ((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return std::ranges::repeat_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::repeat_view<T1, T2> f28(const T1 &a0, T2 a1)
  requires(std::copy_constructible<T1>) &&
          ((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return std::ranges::repeat_view<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::ranges::repeat_view<T1, T2> f29(T1 &&a0, T2 a1)
  requires((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return std::ranges::repeat_view<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
auto f30(const std::ranges::repeat_view<T1, T2> &a0)
  requires((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f31(const std::ranges::repeat_view<T1, T2> &a0)
  requires((!std::same_as<T2, std::unreachable_sentinel_t>)) &&
          ((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return a0.end();
}

template <typename T1, typename T2>
std::unreachable_sentinel_t f32(const std::ranges::repeat_view<T1, T2> &a0)
  requires((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return a0.end();
}

template <typename T1, typename T2>
auto f33(const std::ranges::repeat_view<T1, T2> &a0)
  requires((!std::same_as<T2, std::unreachable_sentinel_t>)) &&
          ((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return a0.size();
}

template <typename T1, typename T2, typename... T3, typename... T4>
std::ranges::repeat_view<T1, T2> f34(std::piecewise_construct_t a0,
                                     std::tuple<T3...> a1, std::tuple<T4...> a2)
  requires((std::constructible_from<T1, T3...> &&
            std::constructible_from<T2, T4...>)) &&
          ((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return std::ranges::repeat_view<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
bool f35(std::ranges::repeat_view<T1, T2> &a0)
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
bool f36(const std::ranges::repeat_view<T1, T2> &a0)
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
bool f37(std::ranges::repeat_view<T1, T2> &a0)
  requires((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
bool f38(const std::ranges::repeat_view<T1, T2> &a0)
  requires((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
auto f39(std::ranges::repeat_view<T1, T2> &a0)
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
auto f40(const std::ranges::repeat_view<T1, T2> &a0)
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
decltype(auto) f41(std::ranges::repeat_view<T1, T2> &a0)
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
decltype(auto) f42(const std::ranges::repeat_view<T1, T2> &a0)
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
decltype(auto) f43(std::ranges::repeat_view<T1, T2> &a0)
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
decltype(auto) f44(const std::ranges::repeat_view<T1, T2> &a0)
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
decltype(auto) f45(std::ranges::repeat_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          ((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return a0.template operator[]<T3>(a1);
}

template <typename T1, typename T2, typename T3>
decltype(auto) f46(const std::ranges::repeat_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          ((std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
            (std::ranges::__integer_like_with_usable_difference_type<T2> ||
             std::same_as<T2, std::unreachable_sentinel_t>)))
{
  return a0.template operator[]<T3>(a1);
}
