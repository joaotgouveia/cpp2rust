#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
std::ranges::filter_view<T1, T2> f8()
  requires((std::default_initializable<T1> &&
            std::default_initializable<T2>)) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return std::ranges::filter_view<T1, T2>();
}

template <typename T1, typename T2>
T1 f9(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::copy_constructible<T1>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.base();
}

template <typename T1, typename T2>
bool f10(std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::filter_view<T1, T2>> &&
           (!std::ranges::sized_range<std::ranges::filter_view<T1, T2>>)) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f11(std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::filter_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f12(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::forward_range<const std::ranges::filter_view<T1, T2>> &&
           (!std::ranges::sized_range<
               const std::ranges::filter_view<T1, T2>>)) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f13(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::sized_range<const std::ranges::filter_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f14(std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::is_object_v<T2>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f15(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::is_object_v<T2>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
auto f16(std::ranges::filter_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.data();
}

template <typename T1, typename T2>
auto f17(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::range<const std::ranges::filter_view<T1, T2>> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               const std::ranges::filter_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.data();
}

template <typename T1, typename T2>
auto f18(std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::filter_view<T1, T2>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<std::ranges::filter_view<T1, T2>>,
               std::ranges::iterator_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.size();
}

template <typename T1, typename T2>
auto f19(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::forward_range<const std::ranges::filter_view<T1, T2>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<const std::ranges::filter_view<T1, T2>>,
               std::ranges::iterator_t<
                   const std::ranges::filter_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.size();
}

template <typename T1, typename T2>
decltype(auto) f20(std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::filter_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f21(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
              const std::ranges::filter_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f22(std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::filter_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::filter_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.back();
}

template <typename T1, typename T2>
decltype(auto) f23(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::filter_view<T1, T2>> &&
           std::ranges::common_range<const std::ranges::filter_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.back();
}

template <typename T1, typename T2>
using t2 = typename std::ranges::filter_view<T1, T2>::_Sentinel;

template <typename T1, typename T2>
typename std::ranges::filter_view<T1, T2>::_Sentinel f24() {
  return typename std::ranges::filter_view<T1, T2>::_Sentinel();
}

template <typename T1, typename T2>
std::ranges::sentinel_t<T1>
f25(const typename std::ranges::filter_view<T1, T2>::_Sentinel &a0) {
  return a0.base();
}

template <typename T1, typename T2>
using t3 = typename std::ranges::filter_view<T1, T2>::_Iterator;

template <typename T1, typename T2>
using t4 =
    typename std::ranges::filter_view<T1, T2>::_Iterator::iterator_concept;

template <typename T1, typename T2>
using t5 = typename std::ranges::filter_view<T1, T2>::_Iterator::value_type;

template <typename T1, typename T2>
using t6 =
    typename std::ranges::filter_view<T1, T2>::_Iterator::difference_type;

template <typename T1, typename T2>
typename std::ranges::filter_view<T1, T2>::_Iterator f26() {
  return typename std::ranges::filter_view<T1, T2>::_Iterator();
}

template <typename T1, typename T2>
const std::ranges::iterator_t<T1> &
f27(const typename std::ranges::filter_view<T1, T2>::_Iterator &a0) {
  return a0.base();
}

template <typename T1, typename T2>
std::ranges::iterator_t<T1>
f28(typename std::ranges::filter_view<T1, T2>::_Iterator &&a0) {
  return std::move(a0).base();
}

template <typename T1, typename T2>
std::ranges::range_reference_t<T1>
f29(const typename std::ranges::filter_view<T1, T2>::_Iterator &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
std::ranges::iterator_t<T1>
f30(const typename std::ranges::filter_view<T1, T2>::_Iterator &a0) {
  return a0.operator->();
}

template <typename T1, typename T2>
typename std::ranges::filter_view<T1, T2>::_Iterator &
f31(typename std::ranges::filter_view<T1, T2>::_Iterator &a0) {
  return a0.operator++();
}

template <typename T1, typename T2>
void f32(typename std::ranges::filter_view<T1, T2>::_Iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
typename std::ranges::filter_view<T1, T2>::_Iterator
f33(typename std::ranges::filter_view<T1, T2>::_Iterator &a0, int a1)
  requires(std::ranges::forward_range<T1>)
{
  return a0.operator++(a1);
}

template <typename T1, typename T2>
typename std::ranges::filter_view<T1, T2>::_Iterator &
f34(typename std::ranges::filter_view<T1, T2>::_Iterator &a0)
  requires(std::ranges::bidirectional_range<T1>)
{
  return a0.operator--();
}

template <typename T1, typename T2>
typename std::ranges::filter_view<T1, T2>::_Iterator
f35(typename std::ranges::filter_view<T1, T2>::_Iterator &a0, int a1)
  requires(std::ranges::bidirectional_range<T1>)
{
  return a0.operator--(a1);
}

template <typename T1, typename T2>
void f36(const typename std::ranges::filter_view<T1, T2>::_Iterator &a0,
         const typename std::ranges::filter_view<T1, T2>::_Iterator &a1) {
  return iter_swap(a0, a1);
}

template <typename T1, typename T2>
bool f37(const typename std::ranges::filter_view<T1, T2>::_Iterator &a0,
         const typename std::ranges::filter_view<T1, T2>::_Iterator &a1) {
  return operator==(a0, a1);
}
