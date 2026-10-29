#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
using t1 = std::ranges::drop_while_view<T1, T2>;

template <typename T1, typename T2>
std::ranges::drop_while_view<T1, T2> f1()
  requires((std::default_initializable<T1> &&
            std::default_initializable<T2>)) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return std::ranges::drop_while_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::drop_while_view<T1, T2> f2(T1 a0, T2 a1)
  requires(std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return std::ranges::drop_while_view<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
T1 f3(const std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::copy_constructible<T1>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.base();
}

template <typename T1, typename T2>
T1 f4(std::ranges::drop_while_view<T1, T2> &&a0)
  requires(std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return std::move(a0).base();
}

template <typename T1, typename T2>
const T2 &f5(const std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.pred();
}

template <typename T1, typename T2>
auto f6(std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f7(std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.end();
}

template <typename T1, typename T2>
bool f8(std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::drop_while_view<T1, T2>> &&
           (!std::ranges::sized_range<std::ranges::drop_while_view<T1, T2>>)) &&
          (std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f9(std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::drop_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f10(const std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
               const std::ranges::drop_while_view<T1, T2>> &&
           (!std::ranges::sized_range<
               const std::ranges::drop_while_view<T1, T2>>)) &&
          (std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f11(const std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::sized_range<
              const std::ranges::drop_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f12(std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f13(const std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
auto f14(std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.data();
}

template <typename T1, typename T2>
auto f15(const std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::range<const std::ranges::drop_while_view<T1, T2>> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               const std::ranges::drop_while_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.data();
}

template <typename T1, typename T2>
auto f16(std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::drop_while_view<T1, T2>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<std::ranges::drop_while_view<T1, T2>>,
               std::ranges::iterator_t<
                   std::ranges::drop_while_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.size();
}

template <typename T1, typename T2>
auto f17(const std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
               const std::ranges::drop_while_view<T1, T2>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<
                   const std::ranges::drop_while_view<T1, T2>>,
               std::ranges::iterator_t<
                   const std::ranges::drop_while_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.size();
}

template <typename T1, typename T2>
decltype(auto) f18(std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::drop_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f19(const std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
              const std::ranges::drop_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f20(std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::drop_while_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::drop_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.back();
}

template <typename T1, typename T2>
decltype(auto) f21(const std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::drop_while_view<T1, T2>> &&
           std::ranges::common_range<
               const std::ranges::drop_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.back();
}

template <typename T1, typename T2>
auto f22(std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::drop_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f23(const std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::input_range<
              const std::ranges::drop_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f24(std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::drop_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.cend();
}

template <typename T1, typename T2>
auto f25(const std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::input_range<
              const std::ranges::drop_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.cend();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::drop_while_view<T1, T2>>
decltype(auto) f26(std::ranges::drop_while_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.template operator[]<T3>(
      std::forward<std::ranges::range_difference_t<T3>>(a1));
}

template <typename T1, typename T2,
          typename T3 = const std::ranges::drop_while_view<T1, T2>>
decltype(auto) f27(const std::ranges::drop_while_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.template operator[]<T3>(
      std::forward<std::ranges::range_difference_t<T3>>(a1));
}
