#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::reverse_view<T1>;

template <typename T1>
std::ranges::reverse_view<T1> f1()
  requires(std::default_initializable<T1>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return std::ranges::reverse_view<T1>();
}

template <typename T1>
std::ranges::reverse_view<T1> f2(T1 a0)
  requires(std::ranges::bidirectional_range<T1>)
{
  return std::ranges::reverse_view<T1>(std::move(a0));
}

template <typename T1>
T1 f3(const std::ranges::reverse_view<T1> &a0)
  requires(std::copy_constructible<T1>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.base();
}

template <typename T1>
T1 f4(std::ranges::reverse_view<T1> &&a0)
  requires(std::ranges::bidirectional_range<T1>)
{
  return std::move(a0).base();
}

template <typename T1>
std::reverse_iterator<std::ranges::iterator_t<T1>>
f5(std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::bidirectional_range<T1>)
{
  return a0.begin();
}

template <typename T1>
auto f6(std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::common_range<T1>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.begin();
}

template <typename T1>
auto f7(const std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::common_range<const T1>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.begin();
}

template <typename T1>
std::reverse_iterator<std::ranges::iterator_t<T1>>
f8(std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::bidirectional_range<T1>)
{
  return a0.end();
}

template <typename T1>
auto f9(const std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::common_range<const T1>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.end();
}

template <typename T1>
auto f10(std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::sized_range<T1>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.size();
}

template <typename T1>
auto f11(const std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::sized_range<const T1>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.size();
}

template <typename T1>
bool f12(std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::reverse_view<T1>> &&
           (!std::ranges::sized_range<std::ranges::reverse_view<T1>>)) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.empty();
}

template <typename T1>
bool f13(std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::sized_range<std::ranges::reverse_view<T1>>) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.empty();
}

template <typename T1>
bool f14(const std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::forward_range<const std::ranges::reverse_view<T1>> &&
           (!std::ranges::sized_range<const std::ranges::reverse_view<T1>>)) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.empty();
}

template <typename T1>
bool f15(const std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::sized_range<const std::ranges::reverse_view<T1>>) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.empty();
}

template <typename T1>
bool f16(std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::bidirectional_range<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1>
bool f17(const std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::bidirectional_range<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1>
auto f18(std::ranges::reverse_view<T1> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::reverse_view<T1>>>) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.data();
}

template <typename T1>
auto f19(const std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::range<const std::ranges::reverse_view<T1>> &&
           std::contiguous_iterator<
               std::ranges::iterator_t<const std::ranges::reverse_view<T1>>>) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.data();
}

template <typename T1>
decltype(auto) f20(std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::reverse_view<T1>>) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f21(const std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::forward_range<const std::ranges::reverse_view<T1>>) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f22(std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::reverse_view<T1>> &&
           std::ranges::common_range<std::ranges::reverse_view<T1>>) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.back();
}

template <typename T1>
decltype(auto) f23(const std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::reverse_view<T1>> &&
           std::ranges::common_range<const std::ranges::reverse_view<T1>>) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.back();
}

template <typename T1>
auto f24(std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::input_range<std::ranges::reverse_view<T1>>) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.cbegin();
}

template <typename T1>
auto f25(const std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::input_range<const std::ranges::reverse_view<T1>>) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.cbegin();
}

template <typename T1>
auto f26(std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::input_range<std::ranges::reverse_view<T1>>) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.cend();
}

template <typename T1>
auto f27(const std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::input_range<const std::ranges::reverse_view<T1>>) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.cend();
}

template <typename T1, typename T2 = std::ranges::reverse_view<T1>>
decltype(auto) f28(std::ranges::reverse_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::random_access_range<T2>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.template operator[]<T2>(
      std::forward<std::ranges::range_difference_t<T2>>(a1));
}

template <typename T1, typename T2 = const std::ranges::reverse_view<T1>>
decltype(auto) f29(const std::ranges::reverse_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::random_access_range<T2>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.template operator[]<T2>(
      std::forward<std::ranges::range_difference_t<T2>>(a1));
}
