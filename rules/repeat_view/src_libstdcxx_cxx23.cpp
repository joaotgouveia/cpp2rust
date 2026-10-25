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
  return std::ranges::repeat_view<T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::ranges::repeat_view<T1, T2> f3(T1 &&a0, T2 a1)
  requires(std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return std::ranges::repeat_view<T1, T2>(std::move(a0), std::move(a1));
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
  return std::ranges::repeat_view<T1, T2>(std::move(a0), std::move(a1),
                                          std::move(a2));
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

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
decltype(auto) f25(std::ranges::repeat_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return a0.template operator[]<T3>(std::move(a1));
}

template <typename T1, typename T2,
          typename T3 = const std::ranges::repeat_view<T1, T2>>
decltype(auto) f26(const std::ranges::repeat_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::is_object_v<T1> && std::same_as<T1, std::remove_cv_t<T1>> &&
           (std::ranges::__detail::__is_integer_like<T2> ||
            std::same_as<T2, std::unreachable_sentinel_t>))
{
  return a0.template operator[]<T3>(std::move(a1));
}

template <typename T1, typename T2>
using t2 = typename std::ranges::repeat_view<T1, T2>::_Iterator;

template <typename T1, typename T2>
using t3 =
    typename std::ranges::repeat_view<T1, T2>::_Iterator::iterator_concept;

template <typename T1, typename T2>
using t4 =
    typename std::ranges::repeat_view<T1, T2>::_Iterator::iterator_category;

template <typename T1, typename T2>
using t5 = typename std::ranges::repeat_view<T1, T2>::_Iterator::value_type;

template <typename T1, typename T2>
using t6 =
    typename std::ranges::repeat_view<T1, T2>::_Iterator::difference_type;

template <typename T1, typename T2>
typename std::ranges::repeat_view<T1, T2>::_Iterator f27() {
  return typename std::ranges::repeat_view<T1, T2>::_Iterator();
}

template <typename T1, typename T2>
const T1 &f28(const typename std::ranges::repeat_view<T1, T2>::_Iterator &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
typename std::ranges::repeat_view<T1, T2>::_Iterator &
f29(typename std::ranges::repeat_view<T1, T2>::_Iterator &a0) {
  return a0.operator++();
}

template <typename T1, typename T2>
typename std::ranges::repeat_view<T1, T2>::_Iterator
f30(typename std::ranges::repeat_view<T1, T2>::_Iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
typename std::ranges::repeat_view<T1, T2>::_Iterator &
f31(typename std::ranges::repeat_view<T1, T2>::_Iterator &a0) {
  return a0.operator--();
}

template <typename T1, typename T2>
typename std::ranges::repeat_view<T1, T2>::_Iterator
f32(typename std::ranges::repeat_view<T1, T2>::_Iterator &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1, typename T2>
typename std::ranges::repeat_view<T1, T2>::_Iterator &
f33(typename std::ranges::repeat_view<T1, T2>::_Iterator &a0,
    typename std::ranges::repeat_view<T1, T2>::_Iterator::difference_type a1) {
  return a0.operator+=(std::move(a1));
}

template <typename T1, typename T2>
typename std::ranges::repeat_view<T1, T2>::_Iterator &
f34(typename std::ranges::repeat_view<T1, T2>::_Iterator &a0,
    typename std::ranges::repeat_view<T1, T2>::_Iterator::difference_type a1) {
  return a0.operator-=(std::move(a1));
}

template <typename T1, typename T2>
const T1 &
f35(const typename std::ranges::repeat_view<T1, T2>::_Iterator &a0,
    typename std::ranges::repeat_view<T1, T2>::_Iterator::difference_type a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2>
typename std::ranges::repeat_view<T1, T2>::_Iterator::difference_type
f36(const typename std::ranges::repeat_view<T1, T2>::_Iterator &a0,
    const typename std::ranges::repeat_view<T1, T2>::_Iterator &a1) {
  return operator-(a0, a1);
}

template <typename T1, typename T2>
typename std::ranges::repeat_view<T1, T2>::_Iterator
f37(typename std::ranges::repeat_view<T1, T2>::_Iterator a0,
    typename std::ranges::repeat_view<T1, T2>::_Iterator::difference_type a1) {
  return operator-(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
typename std::ranges::repeat_view<T1, T2>::_Iterator
f38(typename std::ranges::repeat_view<T1, T2>::_Iterator::difference_type a0,
    typename std::ranges::repeat_view<T1, T2>::_Iterator a1) {
  return operator+(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
typename std::ranges::repeat_view<T1, T2>::_Iterator
f39(typename std::ranges::repeat_view<T1, T2>::_Iterator a0,
    typename std::ranges::repeat_view<T1, T2>::_Iterator::difference_type a1) {
  return operator+(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
auto f40(const typename std::ranges::repeat_view<T1, T2>::_Iterator &a0,
         const typename std::ranges::repeat_view<T1, T2>::_Iterator &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, typename T2>
bool f41(const typename std::ranges::repeat_view<T1, T2>::_Iterator &a0,
         const typename std::ranges::repeat_view<T1, T2>::_Iterator &a1) {
  return operator==(a0, a1);
}
