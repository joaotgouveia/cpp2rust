#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
std::ranges::repeat_view<T1, T2> f1()
  requires(std::default_initializable<T1>) &&
          (std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return std::ranges::repeat_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::repeat_view<T1, T2> f2(const T1 &a0, T2 a1)
  requires(std::copy_constructible<T1>) &&
          (std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return std::ranges::repeat_view<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::ranges::repeat_view<T1, T2> f3(T1 &&a0, T2 a1)
  requires(std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return std::ranges::repeat_view<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
auto f4(const std::ranges::repeat_view<T1, T2> &a0)
  requires(std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f5(const std::ranges::repeat_view<T1, T2> &a0)
  requires((!std::same_as<T2, std::unreachable_sentinel_t>)) &&
          (std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return a0.end();
}

template <typename T1, typename T2>
std::unreachable_sentinel_t f6(const std::ranges::repeat_view<T1, T2> &a0)
  requires(std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return a0.end();
}

template <typename T1, typename T2>
auto f7(const std::ranges::repeat_view<T1, T2> &a0)
  requires((!std::same_as<T2, std::unreachable_sentinel_t>)) &&
          (std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return a0.size();
}

template <typename T1, typename T2, typename... T3, typename... T4>
std::ranges::repeat_view<T1, T2> f8(std::piecewise_construct_t a0,
                                    std::tuple<T3...> a1, std::tuple<T4...> a2)
  requires(std::constructible_from<T1, T3...> &&
           std::constructible_from<T2, T4...>) &&
          (std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return std::ranges::repeat_view<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2>
bool f9(std::ranges::repeat_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::repeat_view<T1, T2>> &&
           (!std::ranges::sized_range<std::ranges::repeat_view<T1, T2>>)) &&
          (std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          (std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f10(std::ranges::repeat_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::repeat_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          (std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f11(const std::ranges::repeat_view<T1, T2> &a0)
  requires(std::ranges::forward_range<const std::ranges::repeat_view<T1, T2>> &&
           (!std::ranges::sized_range<
               const std::ranges::repeat_view<T1, T2>>)) &&
          (std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          (std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f12(const std::ranges::repeat_view<T1, T2> &a0)
  requires(std::ranges::sized_range<const std::ranges::repeat_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          (std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f13(std::ranges::repeat_view<T1, T2> &a0)
  requires(std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f14(const std::ranges::repeat_view<T1, T2> &a0)
  requires(std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
auto f15(std::ranges::repeat_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::repeat_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          (std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return a0.data();
}

template <typename T1, typename T2>
auto f16(const std::ranges::repeat_view<T1, T2> &a0)
  requires(std::ranges::range<const std::ranges::repeat_view<T1, T2>> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               const std::ranges::repeat_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          (std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return a0.data();
}

template <typename T1, typename T2>
decltype(auto) f17(std::ranges::repeat_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::repeat_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          (std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f18(const std::ranges::repeat_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
              const std::ranges::repeat_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          (std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f19(std::ranges::repeat_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::repeat_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::repeat_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          (std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return a0.back();
}

template <typename T1, typename T2>
decltype(auto) f20(const std::ranges::repeat_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::repeat_view<T1, T2>> &&
           std::ranges::common_range<const std::ranges::repeat_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          (std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return a0.back();
}

template <typename T1, typename T2>
auto f21(std::ranges::repeat_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::repeat_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          (std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f22(const std::ranges::repeat_view<T1, T2> &a0)
  requires(std::ranges::input_range<const std::ranges::repeat_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          (std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f23(std::ranges::repeat_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::repeat_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          (std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return a0.cend();
}

template <typename T1, typename T2>
auto f24(const std::ranges::repeat_view<T1, T2> &a0)
  requires(std::ranges::input_range<const std::ranges::repeat_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          (std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f25(std::ranges::repeat_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          (std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return a0.template operator[]<T3>(a1);
}

template <typename T1, typename T2, typename T3>
decltype(auto) f26(const std::ranges::repeat_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          (std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return a0.template operator[]<T3>(a1);
}
