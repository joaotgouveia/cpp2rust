#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, std::size_t T2>
std::ranges::elements_view<T1, T2> f3()
  requires(std::default_initializable<T1>) &&
          (std::ranges::view<T1> &&
           std::ranges::__detail::__has_tuple_element<
               std::ranges::range_value_t<T1>, T2> &&
           std::ranges::__detail::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__detail::__returnable_element<
               std::ranges::range_reference_t<T1>, T2>)
{
  return std::ranges::elements_view<T1, T2>();
}

template <typename T1, std::size_t T2>
std::ranges::elements_view<T1, T2> f4(T1 a0)
  requires(
      std::ranges::view<T1> &&
      std::ranges::__detail::__has_tuple_element<std::ranges::range_value_t<T1>,
                                                 T2> &&
      std::ranges::__detail::__has_tuple_element<
          std::remove_reference_t<std::ranges::range_reference_t<T1>>, T2> &&
      std::ranges::__detail::__returnable_element<
          std::ranges::range_reference_t<T1>, T2>)
{
  return std::ranges::elements_view<T1, T2>(a0);
}

template <typename T1, std::size_t T2>
T1 f5(const std::ranges::elements_view<T1, T2> &a0)
  requires(std::copy_constructible<T1>) &&
          (std::ranges::view<T1> &&
           std::ranges::__detail::__has_tuple_element<
               std::ranges::range_value_t<T1>, T2> &&
           std::ranges::__detail::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__detail::__returnable_element<
               std::ranges::range_reference_t<T1>, T2>)
{
  return a0.base();
}

template <typename T1, std::size_t T2>
T1 f6(std::ranges::elements_view<T1, T2> &&a0)
  requires(
      std::ranges::view<T1> &&
      std::ranges::__detail::__has_tuple_element<std::ranges::range_value_t<T1>,
                                                 T2> &&
      std::ranges::__detail::__has_tuple_element<
          std::remove_reference_t<std::ranges::range_reference_t<T1>>, T2> &&
      std::ranges::__detail::__returnable_element<
          std::ranges::range_reference_t<T1>, T2>)
{
  return std::move(a0).base();
}

template <typename T1, std::size_t T2>
auto f7(std::ranges::elements_view<T1, T2> &a0)
  requires((!std::ranges::__detail::__simple_view<T1>)) &&
          (std::ranges::view<T1> &&
           std::ranges::__detail::__has_tuple_element<
               std::ranges::range_value_t<T1>, T2> &&
           std::ranges::__detail::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__detail::__returnable_element<
               std::ranges::range_reference_t<T1>, T2>)
{
  return a0.begin();
}

template <typename T1, std::size_t T2>
auto f8(const std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::range<const T1>) &&
          (std::ranges::view<T1> &&
           std::ranges::__detail::__has_tuple_element<
               std::ranges::range_value_t<T1>, T2> &&
           std::ranges::__detail::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__detail::__returnable_element<
               std::ranges::range_reference_t<T1>, T2>)
{
  return a0.begin();
}

template <typename T1, std::size_t T2>
auto f9(std::ranges::elements_view<T1, T2> &a0)
  requires((!std::ranges::__detail::__simple_view<T1> &&
            !std::ranges::common_range<T1>)) &&
          (std::ranges::view<T1> &&
           std::ranges::__detail::__has_tuple_element<
               std::ranges::range_value_t<T1>, T2> &&
           std::ranges::__detail::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__detail::__returnable_element<
               std::ranges::range_reference_t<T1>, T2>)
{
  return a0.end();
}

template <typename T1, std::size_t T2>
auto f10(std::ranges::elements_view<T1, T2> &a0)
  requires((!std::ranges::__detail::__simple_view<T1> &&
            std::ranges::common_range<T1>)) &&
          (std::ranges::view<T1> &&
           std::ranges::__detail::__has_tuple_element<
               std::ranges::range_value_t<T1>, T2> &&
           std::ranges::__detail::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__detail::__returnable_element<
               std::ranges::range_reference_t<T1>, T2>)
{
  return a0.end();
}

template <typename T1, std::size_t T2>
auto f11(const std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::range<const T1>) &&
          (std::ranges::view<T1> &&
           std::ranges::__detail::__has_tuple_element<
               std::ranges::range_value_t<T1>, T2> &&
           std::ranges::__detail::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__detail::__returnable_element<
               std::ranges::range_reference_t<T1>, T2>)
{
  return a0.end();
}

template <typename T1, std::size_t T2>
auto f12(const std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::common_range<const T1>) &&
          (std::ranges::view<T1> &&
           std::ranges::__detail::__has_tuple_element<
               std::ranges::range_value_t<T1>, T2> &&
           std::ranges::__detail::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__detail::__returnable_element<
               std::ranges::range_reference_t<T1>, T2>)
{
  return a0.end();
}

template <typename T1, std::size_t T2>
auto f13(std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::sized_range<T1>) &&
          (std::ranges::view<T1> &&
           std::ranges::__detail::__has_tuple_element<
               std::ranges::range_value_t<T1>, T2> &&
           std::ranges::__detail::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__detail::__returnable_element<
               std::ranges::range_reference_t<T1>, T2>)
{
  return a0.size();
}

template <typename T1, std::size_t T2>
auto f14(const std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::sized_range<const T1>) &&
          (std::ranges::view<T1> &&
           std::ranges::__detail::__has_tuple_element<
               std::ranges::range_value_t<T1>, T2> &&
           std::ranges::__detail::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__detail::__returnable_element<
               std::ranges::range_reference_t<T1>, T2>)
{
  return a0.size();
}

template <typename T1, std::size_t T2>
bool f15(std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::elements_view<T1, T2>> &&
           (!std::ranges::sized_range<std::ranges::elements_view<T1, T2>>)) &&
          (std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>>)
{
  return a0.empty();
}

template <typename T1, std::size_t T2>
bool f16(std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::elements_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>>)
{
  return a0.empty();
}

template <typename T1, std::size_t T2>
bool f17(const std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
               const std::ranges::elements_view<T1, T2>> &&
           (!std::ranges::sized_range<
               const std::ranges::elements_view<T1, T2>>)) &&
          (std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>>)
{
  return a0.empty();
}

template <typename T1, std::size_t T2>
bool f18(const std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::sized_range<
              const std::ranges::elements_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>>)
{
  return a0.empty();
}

template <typename T1, std::size_t T2>
bool f19(std::ranges::elements_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, std::size_t T2>
bool f20(const std::ranges::elements_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, std::size_t T2>
auto f21(std::ranges::elements_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::elements_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>>)
{
  return a0.data();
}

template <typename T1, std::size_t T2>
auto f22(const std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::range<const std::ranges::elements_view<T1, T2>> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               const std::ranges::elements_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>>)
{
  return a0.data();
}

template <typename T1, std::size_t T2>
decltype(auto) f23(std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::elements_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>>)
{
  return a0.front();
}

template <typename T1, std::size_t T2>
decltype(auto) f24(const std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
              const std::ranges::elements_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>>)
{
  return a0.front();
}

template <typename T1, std::size_t T2>
decltype(auto) f25(std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::elements_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::elements_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>>)
{
  return a0.back();
}

template <typename T1, std::size_t T2>
decltype(auto) f26(const std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::elements_view<T1, T2>> &&
           std::ranges::common_range<
               const std::ranges::elements_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>>)
{
  return a0.back();
}
