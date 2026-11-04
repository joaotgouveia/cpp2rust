#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::owning_view<T1>;

template <typename T1>
std::ranges::owning_view<T1> f1()
  requires((std::default_initializable<T1>) && std::movable<T1>)
{
  return std::ranges::owning_view<T1>();
}

template <typename T1>
std::ranges::owning_view<T1> f2(T1 &&a0)
  requires(std::movable<T1>)
{
  return std::ranges::owning_view<T1>(std::move(a0));
}

template <typename T1>
std::ranges::owning_view<T1> f3(std::ranges::owning_view<T1> &&a0)
  requires(std::movable<T1>)
{
  return std::ranges::owning_view<T1>(std::move(a0));
}

template <typename T1>
std::ranges::owning_view<T1> &f4(std::ranges::owning_view<T1> &a0,
                                 std::ranges::owning_view<T1> &&a1)
  requires(std::movable<T1>)
{
  return a0.operator=(std::move(a1));
}

template <typename T1>
T1 &f5(std::ranges::owning_view<T1> &a0)
  requires(std::movable<T1>)
{
  return a0.base();
}

template <typename T1>
const T1 &f6(const std::ranges::owning_view<T1> &a0)
  requires(std::movable<T1>)
{
  return a0.base();
}

template <typename T1>
T1 &&f7(std::ranges::owning_view<T1> &&a0)
  requires(std::movable<T1>)
{
  return std::move(a0).base();
}

template <typename T1>
const T1 &&f8(const std::ranges::owning_view<T1> &&a0)
  requires(std::movable<T1>)
{
  return std::move(a0).base();
}

template <typename T1>
std::ranges::iterator_t<T1> f9(std::ranges::owning_view<T1> &a0)
  requires(std::movable<T1>)
{
  return a0.begin();
}

template <typename T1>
std::ranges::sentinel_t<T1> f10(std::ranges::owning_view<T1> &a0)
  requires(std::movable<T1>)
{
  return a0.end();
}

template <typename T1>
auto f11(const std::ranges::owning_view<T1> &a0)
  requires((std::ranges::range<const T1>) && std::movable<T1>)
{
  return a0.begin();
}

template <typename T1>
auto f12(const std::ranges::owning_view<T1> &a0)
  requires((std::ranges::range<const T1>) && std::movable<T1>)
{
  return a0.end();
}

template <typename T1> bool f13(std::ranges::owning_view<T1> &a0) {
  return a0.empty();
}

template <typename T1> bool f14(const std::ranges::owning_view<T1> &a0) {
  return a0.empty();
}

template <typename T1>
auto f15(std::ranges::owning_view<T1> &a0)
  requires((std::ranges::sized_range<T1>) && std::movable<T1>)
{
  return a0.size();
}

template <typename T1>
auto f16(const std::ranges::owning_view<T1> &a0)
  requires((std::ranges::sized_range<const T1>) && std::movable<T1>)
{
  return a0.size();
}

template <typename T1>
auto f17(std::ranges::owning_view<T1> &a0)
  requires((std::ranges::contiguous_range<T1>) && std::movable<T1>)
{
  return a0.data();
}

template <typename T1>
auto f18(const std::ranges::owning_view<T1> &a0)
  requires((std::ranges::contiguous_range<const T1>) && std::movable<T1>)
{
  return a0.data();
}

template <typename T1>
bool f19(std::ranges::owning_view<T1> &a0)
  requires(std::movable<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1>
bool f20(const std::ranges::owning_view<T1> &a0)
  requires(std::movable<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1>
decltype(auto) f21(std::ranges::owning_view<T1> &a0)
  requires((std::ranges::forward_range<std::ranges::owning_view<T1>>) &&
           std::is_class_v<std::ranges::owning_view<T1>> &&
           std::same_as<std::ranges::owning_view<T1>,
                        std::remove_cv_t<std::ranges::owning_view<T1>>> &&
           std::movable<T1>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f22(const std::ranges::owning_view<T1> &a0)
  requires((std::ranges::forward_range<const std::ranges::owning_view<T1>>) &&
           std::is_class_v<std::ranges::owning_view<T1>> &&
           std::same_as<std::ranges::owning_view<T1>,
                        std::remove_cv_t<std::ranges::owning_view<T1>>> &&
           std::movable<T1>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f23(std::ranges::owning_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::owning_view<T1>> &&
           std::ranges::common_range<std::ranges::owning_view<T1>> &&
           std::is_class_v<std::ranges::owning_view<T1>> &&
           std::same_as<std::ranges::owning_view<T1>,
                        std::remove_cv_t<std::ranges::owning_view<T1>>> &&
           std::movable<T1>)
{
  return a0.back();
}

template <typename T1>
decltype(auto) f24(const std::ranges::owning_view<T1> &a0)
  requires(
      std::ranges::bidirectional_range<const std::ranges::owning_view<T1>> &&
      std::ranges::common_range<const std::ranges::owning_view<T1>> &&
      std::is_class_v<std::ranges::owning_view<T1>> &&
      std::same_as<std::ranges::owning_view<T1>,
                   std::remove_cv_t<std::ranges::owning_view<T1>>> &&
      std::movable<T1>)
{
  return a0.back();
}

template <typename T1>
auto f25(std::ranges::owning_view<T1> &a0)
  requires((std::ranges::input_range<std::ranges::owning_view<T1>>) &&
           std::is_class_v<std::ranges::owning_view<T1>> &&
           std::same_as<std::ranges::owning_view<T1>,
                        std::remove_cv_t<std::ranges::owning_view<T1>>> &&
           std::movable<T1>)
{
  return a0.cbegin();
}

template <typename T1>
auto f26(const std::ranges::owning_view<T1> &a0)
  requires((std::ranges::input_range<const std::ranges::owning_view<T1>>) &&
           std::is_class_v<std::ranges::owning_view<T1>> &&
           std::same_as<std::ranges::owning_view<T1>,
                        std::remove_cv_t<std::ranges::owning_view<T1>>> &&
           std::movable<T1>)
{
  return a0.cbegin();
}

template <typename T1>
auto f27(std::ranges::owning_view<T1> &a0)
  requires((std::ranges::input_range<std::ranges::owning_view<T1>>) &&
           std::is_class_v<std::ranges::owning_view<T1>> &&
           std::same_as<std::ranges::owning_view<T1>,
                        std::remove_cv_t<std::ranges::owning_view<T1>>> &&
           std::movable<T1>)
{
  return a0.cend();
}

template <typename T1>
auto f28(const std::ranges::owning_view<T1> &a0)
  requires((std::ranges::input_range<const std::ranges::owning_view<T1>>) &&
           std::is_class_v<std::ranges::owning_view<T1>> &&
           std::same_as<std::ranges::owning_view<T1>,
                        std::remove_cv_t<std::ranges::owning_view<T1>>> &&
           std::movable<T1>)
{
  return a0.cend();
}

template <typename T1, typename T2 = std::ranges::owning_view<T1>>
decltype(auto) f29(std::ranges::owning_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::owning_view<T1>> &&
           std::same_as<std::ranges::owning_view<T1>,
                        std::remove_cv_t<std::ranges::owning_view<T1>>> &&
           (std::ranges::random_access_range<T2>) && std::movable<T1>)
{
  return a0.template operator[]<T2>(
      std::forward<std::ranges::range_difference_t<T2>>(a1));
}

template <typename T1, typename T2 = const std::ranges::owning_view<T1>>
decltype(auto) f30(const std::ranges::owning_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::owning_view<T1>> &&
           std::same_as<std::ranges::owning_view<T1>,
                        std::remove_cv_t<std::ranges::owning_view<T1>>> &&
           (std::ranges::random_access_range<T2>) && std::movable<T1>)
{
  return a0.template operator[]<T2>(
      std::forward<std::ranges::range_difference_t<T2>>(a1));
}
