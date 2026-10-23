#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
std::ranges::join_with_view<T1, T2> f3()
  requires((std::default_initializable<T1> &&
            std::default_initializable<T2>)) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::__detail::__compatible_joinable_ranges<
               std::ranges::range_reference_t<T1>, T2>)
{
  return std::ranges::join_with_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::join_with_view<T1, T2> f4(T1 a0, T2 a1)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::__detail::__compatible_joinable_ranges<
               std::ranges::range_reference_t<T1>, T2>)
{
  return std::ranges::join_with_view<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
T1 f5(const std::ranges::join_with_view<T1, T2> &a0)
  requires(std::copy_constructible<T1>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::__detail::__compatible_joinable_ranges<
               std::ranges::range_reference_t<T1>, T2>)
{
  return a0.base();
}

template <typename T1, typename T2>
T1 f6(std::ranges::join_with_view<T1, T2> &&a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::__detail::__compatible_joinable_ranges<
               std::ranges::range_reference_t<T1>, T2>)
{
  return std::move(a0).base();
}

template <typename T1, typename T2>
auto f7(std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::__detail::__compatible_joinable_ranges<
               std::ranges::range_reference_t<T1>, T2>)
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f8(const std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::forward_range<const T1> &&
           std::ranges::forward_range<const T2> &&
           std::is_reference_v<std::ranges::range_reference_t<const T1>> &&
           std::ranges::input_range<
               std::ranges::range_reference_t<const T1>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::__detail::__compatible_joinable_ranges<
               std::ranges::range_reference_t<T1>, T2>)
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f9(std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::__detail::__compatible_joinable_ranges<
               std::ranges::range_reference_t<T1>, T2>)
{
  return a0.end();
}

template <typename T1, typename T2>
auto f10(const std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::forward_range<const T1> &&
           std::ranges::forward_range<const T2> &&
           std::is_reference_v<std::ranges::range_reference_t<const T1>> &&
           std::ranges::input_range<
               std::ranges::range_reference_t<const T1>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::__detail::__compatible_joinable_ranges<
               std::ranges::range_reference_t<T1>, T2>)
{
  return a0.end();
}

template <typename T1, typename T2, typename T3>
std::ranges::join_with_view<T1, T2>
f11(T3 &&a0, std::ranges::range_value_t<std::ranges::range_reference_t<T1>> a1)
  requires(std::constructible_from<T1, std::ranges::views::all_t<T3>> &&
           std::constructible_from<
               T2, std::ranges::single_view<std::ranges::range_value_t<
                       typename std::ranges::join_with_view<
                           T1, T2>::_InnerRange>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::__detail::__compatible_joinable_ranges<
               std::ranges::range_reference_t<T1>, T2>)
{
  return std::ranges::join_with_view<T1, T2>(std::forward<T3>(a0), a1);
}

template <typename T1, typename T2>
bool f12(std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::join_with_view<T1, T2>> &&
           (!std::ranges::sized_range<std::ranges::join_with_view<T1, T2>>)) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<std::ranges::join_with_view<T1, T2>,
                        std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f13(std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::join_with_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<std::ranges::join_with_view<T1, T2>,
                        std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f14(const std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
               const std::ranges::join_with_view<T1, T2>> &&
           (!std::ranges::sized_range<
               const std::ranges::join_with_view<T1, T2>>)) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<std::ranges::join_with_view<T1, T2>,
                        std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f15(const std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::sized_range<
              const std::ranges::join_with_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<std::ranges::join_with_view<T1, T2>,
                        std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f16(std::ranges::join_with_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f17(const std::ranges::join_with_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
auto f18(std::ranges::join_with_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::join_with_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<std::ranges::join_with_view<T1, T2>,
                        std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>)
{
  return a0.data();
}

template <typename T1, typename T2>
auto f19(const std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::range<const std::ranges::join_with_view<T1, T2>> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               const std::ranges::join_with_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<std::ranges::join_with_view<T1, T2>,
                        std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>)
{
  return a0.data();
}

template <typename T1, typename T2>
auto f20(std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::join_with_view<T1, T2>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<std::ranges::join_with_view<T1, T2>>,
               std::ranges::iterator_t<std::ranges::join_with_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<std::ranges::join_with_view<T1, T2>,
                        std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>)
{
  return a0.size();
}

template <typename T1, typename T2>
auto f21(const std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
               const std::ranges::join_with_view<T1, T2>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<
                   const std::ranges::join_with_view<T1, T2>>,
               std::ranges::iterator_t<
                   const std::ranges::join_with_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<std::ranges::join_with_view<T1, T2>,
                        std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>)
{
  return a0.size();
}

template <typename T1, typename T2>
decltype(auto) f22(std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::join_with_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<std::ranges::join_with_view<T1, T2>,
                        std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>)
{
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f23(const std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
              const std::ranges::join_with_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<std::ranges::join_with_view<T1, T2>,
                        std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>)
{
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f24(std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::join_with_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::join_with_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<std::ranges::join_with_view<T1, T2>,
                        std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>)
{
  return a0.back();
}

template <typename T1, typename T2>
decltype(auto) f25(const std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::join_with_view<T1, T2>> &&
           std::ranges::common_range<
               const std::ranges::join_with_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<std::ranges::join_with_view<T1, T2>,
                        std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>)
{
  return a0.back();
}

template <typename T1, typename T2>
auto f26(std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::join_with_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<std::ranges::join_with_view<T1, T2>,
                        std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f27(const std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::input_range<
              const std::ranges::join_with_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<std::ranges::join_with_view<T1, T2>,
                        std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f28(std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::join_with_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<std::ranges::join_with_view<T1, T2>,
                        std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>)
{
  return a0.cend();
}

template <typename T1, typename T2>
auto f29(const std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::input_range<
              const std::ranges::join_with_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<std::ranges::join_with_view<T1, T2>,
                        std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>)
{
  return a0.cend();
}
