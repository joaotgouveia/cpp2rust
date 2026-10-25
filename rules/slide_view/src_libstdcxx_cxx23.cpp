#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::slide_view<T1>;

template <typename T1>
std::ranges::slide_view<T1> f1(T1 a0, std::ranges::range_difference_t<T1> a1)
  requires(std::ranges::view<T1>)
{
  return std::ranges::slide_view<T1>(std::move(a0), std::move(a1));
}

template <typename T1>
T1 f2(const std::ranges::slide_view<T1> &a0)
  requires(std::copy_constructible<T1>) && (std::ranges::view<T1>)
{
  return a0.base();
}

template <typename T1>
T1 f3(std::ranges::slide_view<T1> &&a0)
  requires(std::ranges::view<T1>)
{
  return std::move(a0).base();
}

template <typename T1>
auto f4(std::ranges::slide_view<T1> &a0)
  requires((!(std::ranges::__detail::__simple_view<T1> &&
              std::ranges::__detail::__slide_caches_nothing<const T1>))) &&
          (std::ranges::view<T1>)
{
  return a0.begin();
}

template <typename T1>
auto f5(const std::ranges::slide_view<T1> &a0)
  requires(std::ranges::__detail::__slide_caches_nothing<const T1>) &&
          (std::ranges::view<T1>)
{
  return a0.begin();
}

template <typename T1>
auto f6(std::ranges::slide_view<T1> &a0)
  requires((!(std::ranges::__detail::__simple_view<T1> &&
              std::ranges::__detail::__slide_caches_nothing<const T1>))) &&
          (std::ranges::view<T1>)
{
  return a0.end();
}

template <typename T1>
auto f7(const std::ranges::slide_view<T1> &a0)
  requires(std::ranges::__detail::__slide_caches_nothing<const T1>) &&
          (std::ranges::view<T1>)
{
  return a0.end();
}

template <typename T1>
auto f8(std::ranges::slide_view<T1> &a0)
  requires(std::ranges::sized_range<T1>) && (std::ranges::view<T1>)
{
  return a0.size();
}

template <typename T1>
auto f9(const std::ranges::slide_view<T1> &a0)
  requires(std::ranges::sized_range<const T1>) && (std::ranges::view<T1>)
{
  return a0.size();
}

template <typename T1>
bool f10(std::ranges::slide_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::slide_view<T1>> &&
           (!std::ranges::sized_range<std::ranges::slide_view<T1>>)) &&
          (std::is_class_v<std::ranges::slide_view<T1>> &&
           std::same_as<std::ranges::slide_view<T1>,
                        std::remove_cv_t<std::ranges::slide_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.empty();
}

template <typename T1>
bool f11(std::ranges::slide_view<T1> &a0)
  requires(std::ranges::sized_range<std::ranges::slide_view<T1>>) &&
          (std::is_class_v<std::ranges::slide_view<T1>> &&
           std::same_as<std::ranges::slide_view<T1>,
                        std::remove_cv_t<std::ranges::slide_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.empty();
}

template <typename T1>
bool f12(const std::ranges::slide_view<T1> &a0)
  requires(std::ranges::forward_range<const std::ranges::slide_view<T1>> &&
           (!std::ranges::sized_range<const std::ranges::slide_view<T1>>)) &&
          (std::is_class_v<std::ranges::slide_view<T1>> &&
           std::same_as<std::ranges::slide_view<T1>,
                        std::remove_cv_t<std::ranges::slide_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.empty();
}

template <typename T1>
bool f13(const std::ranges::slide_view<T1> &a0)
  requires(std::ranges::sized_range<const std::ranges::slide_view<T1>>) &&
          (std::is_class_v<std::ranges::slide_view<T1>> &&
           std::same_as<std::ranges::slide_view<T1>,
                        std::remove_cv_t<std::ranges::slide_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.empty();
}

template <typename T1>
bool f14(std::ranges::slide_view<T1> &a0)
  requires(std::ranges::view<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1>
bool f15(const std::ranges::slide_view<T1> &a0)
  requires(std::ranges::view<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1>
auto f16(std::ranges::slide_view<T1> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::slide_view<T1>>>) &&
          (std::is_class_v<std::ranges::slide_view<T1>> &&
           std::same_as<std::ranges::slide_view<T1>,
                        std::remove_cv_t<std::ranges::slide_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.data();
}

template <typename T1>
auto f17(const std::ranges::slide_view<T1> &a0)
  requires(std::ranges::range<const std::ranges::slide_view<T1>> &&
           std::contiguous_iterator<
               std::ranges::iterator_t<const std::ranges::slide_view<T1>>>) &&
          (std::is_class_v<std::ranges::slide_view<T1>> &&
           std::same_as<std::ranges::slide_view<T1>,
                        std::remove_cv_t<std::ranges::slide_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.data();
}

template <typename T1>
decltype(auto) f18(std::ranges::slide_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::slide_view<T1>>) &&
          (std::is_class_v<std::ranges::slide_view<T1>> &&
           std::same_as<std::ranges::slide_view<T1>,
                        std::remove_cv_t<std::ranges::slide_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f19(const std::ranges::slide_view<T1> &a0)
  requires(std::ranges::forward_range<const std::ranges::slide_view<T1>>) &&
          (std::is_class_v<std::ranges::slide_view<T1>> &&
           std::same_as<std::ranges::slide_view<T1>,
                        std::remove_cv_t<std::ranges::slide_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f20(std::ranges::slide_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::slide_view<T1>> &&
           std::ranges::common_range<std::ranges::slide_view<T1>>) &&
          (std::is_class_v<std::ranges::slide_view<T1>> &&
           std::same_as<std::ranges::slide_view<T1>,
                        std::remove_cv_t<std::ranges::slide_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.back();
}

template <typename T1>
decltype(auto) f21(const std::ranges::slide_view<T1> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::slide_view<T1>> &&
           std::ranges::common_range<const std::ranges::slide_view<T1>>) &&
          (std::is_class_v<std::ranges::slide_view<T1>> &&
           std::same_as<std::ranges::slide_view<T1>,
                        std::remove_cv_t<std::ranges::slide_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.back();
}

template <typename T1>
auto f22(std::ranges::slide_view<T1> &a0)
  requires(std::ranges::input_range<std::ranges::slide_view<T1>>) &&
          (std::is_class_v<std::ranges::slide_view<T1>> &&
           std::same_as<std::ranges::slide_view<T1>,
                        std::remove_cv_t<std::ranges::slide_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.cbegin();
}

template <typename T1>
auto f23(const std::ranges::slide_view<T1> &a0)
  requires(std::ranges::input_range<const std::ranges::slide_view<T1>>) &&
          (std::is_class_v<std::ranges::slide_view<T1>> &&
           std::same_as<std::ranges::slide_view<T1>,
                        std::remove_cv_t<std::ranges::slide_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.cbegin();
}

template <typename T1>
auto f24(std::ranges::slide_view<T1> &a0)
  requires(std::ranges::input_range<std::ranges::slide_view<T1>>) &&
          (std::is_class_v<std::ranges::slide_view<T1>> &&
           std::same_as<std::ranges::slide_view<T1>,
                        std::remove_cv_t<std::ranges::slide_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.cend();
}

template <typename T1>
auto f25(const std::ranges::slide_view<T1> &a0)
  requires(std::ranges::input_range<const std::ranges::slide_view<T1>>) &&
          (std::is_class_v<std::ranges::slide_view<T1>> &&
           std::same_as<std::ranges::slide_view<T1>,
                        std::remove_cv_t<std::ranges::slide_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.cend();
}

template <typename T1, typename T2 = std::ranges::slide_view<T1>>
decltype(auto) f26(std::ranges::slide_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::slide_view<T1>> &&
           std::same_as<std::ranges::slide_view<T1>,
                        std::remove_cv_t<std::ranges::slide_view<T1>>>) &&
          (std::ranges::random_access_range<T2>) && (std::ranges::view<T1>)
{
  return a0.template operator[]<T2>(std::move(a1));
}

template <typename T1, typename T2 = const std::ranges::slide_view<T1>>
decltype(auto) f27(const std::ranges::slide_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::slide_view<T1>> &&
           std::same_as<std::ranges::slide_view<T1>,
                        std::remove_cv_t<std::ranges::slide_view<T1>>>) &&
          (std::ranges::random_access_range<T2>) && (std::ranges::view<T1>)
{
  return a0.template operator[]<T2>(std::move(a1));
}

template <typename T1>
using t2 = typename std::ranges::slide_view<T1>::_Sentinel;

template <typename T1> typename std::ranges::slide_view<T1>::_Sentinel f28() {
  return typename std::ranges::slide_view<T1>::_Sentinel();
}
