#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::single_view<T1>;

template <typename T1>
std::ranges::single_view<T1> f1()
  requires(std::default_initializable<T1>) && (std::is_object_v<T1>)
{
  return std::ranges::single_view<T1>();
}

template <typename T1>
std::ranges::single_view<T1> f2(const T1 &a0)
  requires(std::copy_constructible<T1>) && (std::is_object_v<T1>)
{
  return std::ranges::single_view<T1>(a0);
}

template <typename T1>
std::ranges::single_view<T1> f3(T1 &&a0)
  requires(std::is_object_v<T1>)
{
  return std::ranges::single_view<T1>(std::move(a0));
}

template <typename T1>
T1 *f4(std::ranges::single_view<T1> &a0)
  requires(std::is_object_v<T1>)
{
  return a0.begin();
}

template <typename T1>
const T1 *f5(const std::ranges::single_view<T1> &a0)
  requires(std::is_object_v<T1>)
{
  return a0.begin();
}

template <typename T1>
T1 *f6(std::ranges::single_view<T1> &a0)
  requires(std::is_object_v<T1>)
{
  return a0.end();
}

template <typename T1>
const T1 *f7(const std::ranges::single_view<T1> &a0)
  requires(std::is_object_v<T1>)
{
  return a0.end();
}

template <typename T1>
bool f8()
  requires(std::is_object_v<T1>)
{
  return std::ranges::single_view<T1>::empty();
}

template <typename T1>
std::size_t f9()
  requires(std::is_object_v<T1>)
{
  return std::ranges::single_view<T1>::size();
}

template <typename T1>
T1 *f10(std::ranges::single_view<T1> &a0)
  requires(std::is_object_v<T1>)
{
  return a0.data();
}

template <typename T1>
const T1 *f11(const std::ranges::single_view<T1> &a0)
  requires(std::is_object_v<T1>)
{
  return a0.data();
}

template <typename T1, typename... T2>
std::ranges::single_view<T1> f12(std::in_place_t a0, T2 &&...a1)
  requires(std::constructible_from<T1, T2...>) && (std::is_object_v<T1>)
{
  return std::ranges::single_view<T1>(std::move(a0), std::forward<T2>(a1)...);
}

template <typename T1>
bool f13(std::ranges::single_view<T1> &a0)
  requires(std::is_object_v<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1>
bool f14(const std::ranges::single_view<T1> &a0)
  requires(std::is_object_v<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1>
decltype(auto) f15(std::ranges::single_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::single_view<T1>>) &&
          (std::is_class_v<std::ranges::single_view<T1>> &&
           std::same_as<std::ranges::single_view<T1>,
                        std::remove_cv_t<std::ranges::single_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f16(const std::ranges::single_view<T1> &a0)
  requires(std::ranges::forward_range<const std::ranges::single_view<T1>>) &&
          (std::is_class_v<std::ranges::single_view<T1>> &&
           std::same_as<std::ranges::single_view<T1>,
                        std::remove_cv_t<std::ranges::single_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f17(std::ranges::single_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::single_view<T1>> &&
           std::ranges::common_range<std::ranges::single_view<T1>>) &&
          (std::is_class_v<std::ranges::single_view<T1>> &&
           std::same_as<std::ranges::single_view<T1>,
                        std::remove_cv_t<std::ranges::single_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.back();
}

template <typename T1>
decltype(auto) f18(const std::ranges::single_view<T1> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::single_view<T1>> &&
           std::ranges::common_range<const std::ranges::single_view<T1>>) &&
          (std::is_class_v<std::ranges::single_view<T1>> &&
           std::same_as<std::ranges::single_view<T1>,
                        std::remove_cv_t<std::ranges::single_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.back();
}

template <typename T1>
auto f19(std::ranges::single_view<T1> &a0)
  requires(std::ranges::input_range<std::ranges::single_view<T1>>) &&
          (std::is_class_v<std::ranges::single_view<T1>> &&
           std::same_as<std::ranges::single_view<T1>,
                        std::remove_cv_t<std::ranges::single_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.cbegin();
}

template <typename T1>
auto f20(const std::ranges::single_view<T1> &a0)
  requires(std::ranges::input_range<const std::ranges::single_view<T1>>) &&
          (std::is_class_v<std::ranges::single_view<T1>> &&
           std::same_as<std::ranges::single_view<T1>,
                        std::remove_cv_t<std::ranges::single_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.cbegin();
}

template <typename T1>
auto f21(std::ranges::single_view<T1> &a0)
  requires(std::ranges::input_range<std::ranges::single_view<T1>>) &&
          (std::is_class_v<std::ranges::single_view<T1>> &&
           std::same_as<std::ranges::single_view<T1>,
                        std::remove_cv_t<std::ranges::single_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.cend();
}

template <typename T1>
auto f22(const std::ranges::single_view<T1> &a0)
  requires(std::ranges::input_range<const std::ranges::single_view<T1>>) &&
          (std::is_class_v<std::ranges::single_view<T1>> &&
           std::same_as<std::ranges::single_view<T1>,
                        std::remove_cv_t<std::ranges::single_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.cend();
}

template <typename T1, typename T2 = std::ranges::single_view<T1>>
decltype(auto) f23(std::ranges::single_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::single_view<T1>> &&
           std::same_as<std::ranges::single_view<T1>,
                        std::remove_cv_t<std::ranges::single_view<T1>>>) &&
          (std::ranges::random_access_range<T2>) && (std::is_object_v<T1>)
{
  return a0.template operator[]<T2>(
      std::forward<std::ranges::range_difference_t<T2>>(a1));
}

template <typename T1, typename T2 = const std::ranges::single_view<T1>>
decltype(auto) f24(const std::ranges::single_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::single_view<T1>> &&
           std::same_as<std::ranges::single_view<T1>,
                        std::remove_cv_t<std::ranges::single_view<T1>>>) &&
          (std::ranges::random_access_range<T2>) && (std::is_object_v<T1>)
{
  return a0.template operator[]<T2>(
      std::forward<std::ranges::range_difference_t<T2>>(a1));
}
