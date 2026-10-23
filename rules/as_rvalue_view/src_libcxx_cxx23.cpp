#include <ranges>
#include <utility>

template <typename T1>
std::ranges::as_rvalue_view<T1> f29()
  requires(std::default_initializable<T1>) && (std::ranges::input_range<T1>)
{
  return std::ranges::as_rvalue_view<T1>();
}

template <typename T1>
std::ranges::as_rvalue_view<T1> f30(T1 a0)
  requires(std::ranges::input_range<T1>)
{
  return std::ranges::as_rvalue_view<T1>(a0);
}

template <typename T1>
T1 f31(const std::ranges::as_rvalue_view<T1> &a0)
  requires(std::copy_constructible<T1>) && (std::ranges::input_range<T1>)
{
  return a0.base();
}

template <typename T1>
T1 f32(std::ranges::as_rvalue_view<T1> &&a0)
  requires(std::ranges::input_range<T1>)
{
  return std::move(a0).base();
}

template <typename T1>
auto f33(std::ranges::as_rvalue_view<T1> &a0)
  requires((!std::ranges::__simple_view<T1>)) && (std::ranges::input_range<T1>)
{
  return a0.begin();
}

template <typename T1>
auto f34(const std::ranges::as_rvalue_view<T1> &a0)
  requires(std::ranges::range<const T1>) && (std::ranges::input_range<T1>)
{
  return a0.begin();
}

template <typename T1>
auto f35(std::ranges::as_rvalue_view<T1> &a0)
  requires((!std::ranges::__simple_view<T1>)) && (std::ranges::input_range<T1>)
{
  return a0.end();
}

template <typename T1>
auto f36(const std::ranges::as_rvalue_view<T1> &a0)
  requires(std::ranges::range<const T1>) && (std::ranges::input_range<T1>)
{
  return a0.end();
}

template <typename T1>
auto f37(std::ranges::as_rvalue_view<T1> &a0)
  requires(std::ranges::sized_range<T1>) && (std::ranges::input_range<T1>)
{
  return a0.size();
}

template <typename T1>
auto f38(const std::ranges::as_rvalue_view<T1> &a0)
  requires(std::ranges::sized_range<const T1>) && (std::ranges::input_range<T1>)
{
  return a0.size();
}

template <typename T1, typename T2 = std::ranges::as_rvalue_view<T1>>
bool f39(std::ranges::as_rvalue_view<T1> &a0)
  requires(std::ranges::sized_range<std::ranges::as_rvalue_view<T1>> ||
           std::ranges::forward_range<std::ranges::as_rvalue_view<T1>>) &&
          (std::is_class_v<std::ranges::as_rvalue_view<T1>> &&
           std::same_as<std::ranges::as_rvalue_view<T1>,
                        std::remove_cv_t<std::ranges::as_rvalue_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.empty();
}

template <typename T1, typename T2 = std::ranges::as_rvalue_view<T1>>
bool f40(const std::ranges::as_rvalue_view<T1> &a0)
  requires(std::ranges::sized_range<std::ranges::as_rvalue_view<T1> const> ||
           std::ranges::forward_range<std::ranges::as_rvalue_view<T1> const>) &&
          (std::is_class_v<std::ranges::as_rvalue_view<T1>> &&
           std::same_as<std::ranges::as_rvalue_view<T1>,
                        std::remove_cv_t<std::ranges::as_rvalue_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.empty();
}

template <typename T1, typename T2 = std::ranges::as_rvalue_view<T1>>
bool f41(std::ranges::as_rvalue_view<T1> &a0)
  requires(std::ranges::input_range<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::as_rvalue_view<T1>>
bool f42(const std::ranges::as_rvalue_view<T1> &a0)
  requires(std::ranges::input_range<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::as_rvalue_view<T1>>
auto f43(std::ranges::as_rvalue_view<T1> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::as_rvalue_view<T1>>>) &&
          (std::is_class_v<std::ranges::as_rvalue_view<T1>> &&
           std::same_as<std::ranges::as_rvalue_view<T1>,
                        std::remove_cv_t<std::ranges::as_rvalue_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.data();
}

template <typename T1, typename T2 = std::ranges::as_rvalue_view<T1>>
auto f44(const std::ranges::as_rvalue_view<T1> &a0)
  requires(std::ranges::range<std::ranges::as_rvalue_view<T1> const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::as_rvalue_view<T1> const>>) &&
          (std::is_class_v<std::ranges::as_rvalue_view<T1>> &&
           std::same_as<std::ranges::as_rvalue_view<T1>,
                        std::remove_cv_t<std::ranges::as_rvalue_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.data();
}

template <typename T1, typename T2 = std::ranges::as_rvalue_view<T1>>
decltype(auto) f45(std::ranges::as_rvalue_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::as_rvalue_view<T1>>) &&
          (std::is_class_v<std::ranges::as_rvalue_view<T1>> &&
           std::same_as<std::ranges::as_rvalue_view<T1>,
                        std::remove_cv_t<std::ranges::as_rvalue_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::as_rvalue_view<T1>>
decltype(auto) f46(const std::ranges::as_rvalue_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::as_rvalue_view<T1> const>) &&
          (std::is_class_v<std::ranges::as_rvalue_view<T1>> &&
           std::same_as<std::ranges::as_rvalue_view<T1>,
                        std::remove_cv_t<std::ranges::as_rvalue_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::as_rvalue_view<T1>>
decltype(auto) f47(std::ranges::as_rvalue_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::as_rvalue_view<T1>> &&
           std::ranges::common_range<std::ranges::as_rvalue_view<T1>>) &&
          (std::is_class_v<std::ranges::as_rvalue_view<T1>> &&
           std::same_as<std::ranges::as_rvalue_view<T1>,
                        std::remove_cv_t<std::ranges::as_rvalue_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.back();
}

template <typename T1, typename T2 = std::ranges::as_rvalue_view<T1>>
decltype(auto) f48(const std::ranges::as_rvalue_view<T1> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::as_rvalue_view<T1> const> &&
           std::ranges::common_range<std::ranges::as_rvalue_view<T1> const>) &&
          (std::is_class_v<std::ranges::as_rvalue_view<T1>> &&
           std::same_as<std::ranges::as_rvalue_view<T1>,
                        std::remove_cv_t<std::ranges::as_rvalue_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.back();
}

template <typename T1, typename T2>
decltype(auto) f49(std::ranges::as_rvalue_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::as_rvalue_view<T1>> &&
           std::same_as<std::ranges::as_rvalue_view<T1>,
                        std::remove_cv_t<std::ranges::as_rvalue_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.template operator[]<T2>(a1);
}

template <typename T1, typename T2>
decltype(auto) f50(const std::ranges::as_rvalue_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::as_rvalue_view<T1>> &&
           std::same_as<std::ranges::as_rvalue_view<T1>,
                        std::remove_cv_t<std::ranges::as_rvalue_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.template operator[]<T2>(a1);
}
