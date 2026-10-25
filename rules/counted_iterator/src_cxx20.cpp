#include <algorithm>
#include <compare>
#include <iterator>
#include <utility>

template <typename T1> using t1 = std::counted_iterator<T1>;

template <typename T1>
using t2 = typename std::counted_iterator<T1>::iterator_type;

template <typename T1>
using t3 = typename std::counted_iterator<T1>::difference_type;

template <typename T1>
std::counted_iterator<T1> f1()
  requires(std::default_initializable<T1>)
{
  return std::counted_iterator<T1>();
}

template <typename T1>
std::counted_iterator<T1> f2(T1 a0, std::iter_difference_t<T1> a1) {
  return std::counted_iterator<T1>(
      std::move(a0), std::forward<std::iter_difference_t<T1>>(a1));
}

template <typename T1> const T1 &f3(const std::counted_iterator<T1> &a0) {
  return a0.base();
}

template <typename T1> T1 f4(std::counted_iterator<T1> &&a0) {
  return std::move(a0).base();
}

template <typename T1>
std::iter_difference_t<T1> f5(const std::counted_iterator<T1> &a0) {
  return a0.count();
}

template <typename T1> decltype(auto) f6(std::counted_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
auto f7(const std::counted_iterator<T1> &a0)
  requires(std::contiguous_iterator<T1>)
{
  return a0.operator->();
}

template <typename T1>
std::counted_iterator<T1> &f8(std::counted_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
decltype(auto) f9(std::counted_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
std::counted_iterator<T1> f10(std::counted_iterator<T1> &a0, int a1)
  requires(std::forward_iterator<T1>)
{
  return a0.operator++(a1);
}

template <typename T1>
std::counted_iterator<T1> &f11(std::counted_iterator<T1> &a0)
  requires(std::bidirectional_iterator<T1>)
{
  return a0.operator--();
}

template <typename T1>
std::counted_iterator<T1> f12(std::counted_iterator<T1> &a0, int a1)
  requires(std::bidirectional_iterator<T1>)
{
  return a0.operator--(a1);
}

template <typename T1>
std::counted_iterator<T1> f13(const std::counted_iterator<T1> &a0,
                              std::iter_difference_t<T1> a1)
  requires(std::random_access_iterator<T1>)
{
  return a0.operator+(std::forward<std::iter_difference_t<T1>>(a1));
}

template <typename T1>
std::counted_iterator<T1> &f14(std::counted_iterator<T1> &a0,
                               std::iter_difference_t<T1> a1)
  requires(std::random_access_iterator<T1>)
{
  return a0.operator+=(std::forward<std::iter_difference_t<T1>>(a1));
}

template <typename T1>
std::counted_iterator<T1> f15(const std::counted_iterator<T1> &a0,
                              std::iter_difference_t<T1> a1)
  requires(std::random_access_iterator<T1>)
{
  return a0.operator-(std::forward<std::iter_difference_t<T1>>(a1));
}

template <typename T1>
std::counted_iterator<T1> &f16(std::counted_iterator<T1> &a0,
                               std::iter_difference_t<T1> a1)
  requires(std::random_access_iterator<T1>)
{
  return a0.operator-=(std::forward<std::iter_difference_t<T1>>(a1));
}

template <typename T1>
decltype(auto) f17(const std::counted_iterator<T1> &a0,
                   std::iter_difference_t<T1> a1)
  requires(std::random_access_iterator<T1>)
{
  return a0.operator[](std::forward<std::iter_difference_t<T1>>(a1));
}

template <typename T1, typename T2>
std::counted_iterator<T1> f18(const std::counted_iterator<T2> &a0)
  requires(std::convertible_to<const T2 &, T1>)
{
  return std::counted_iterator<T1>(a0);
}

template <typename T1, typename T2>
std::counted_iterator<T1> &f19(std::counted_iterator<T1> &a0,
                               const std::counted_iterator<T2> &a1)
  requires(std::assignable_from<T1 &, const T2 &>)
{
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
void f20(const std::counted_iterator<T1> &a0,
         const std::counted_iterator<T2> &a1)
  requires(std::indirectly_swappable<T2, T1>)
{
  return iter_swap(a0, a1);
}

template <typename T1, typename T2>
std::strong_ordering f21(const std::counted_iterator<T1> &a0,
                         const std::counted_iterator<T2> &a1)
  requires(std::common_with<T2, T1>)
{
  return operator<=>(a0, a1);
}

template <typename T1>
bool f22(const std::counted_iterator<T1> &a0, std::default_sentinel_t a1) {
  return operator==(a0, std::move(a1));
}

template <typename T1, typename T2>
bool f23(const std::counted_iterator<T1> &a0,
         const std::counted_iterator<T2> &a1)
  requires(std::common_with<T2, T1>)
{
  return operator==(a0, a1);
}

template <typename T1>
std::iter_difference_t<T1> f24(std::default_sentinel_t a0,
                               const std::counted_iterator<T1> &a1) {
  return operator-(std::move(a0), a1);
}

template <typename T1>
std::iter_difference_t<T1> f25(const std::counted_iterator<T1> &a0,
                               std::default_sentinel_t a1) {
  return operator-(a0, std::move(a1));
}

template <typename T1, typename T2>
std::iter_difference_t<T2> f26(const std::counted_iterator<T1> &a0,
                               const std::counted_iterator<T2> &a1)
  requires(std::common_with<T2, T1>)
{
  return operator-(a0, a1);
}

template <typename T1>
std::counted_iterator<T1> f27(std::iter_difference_t<T1> a0,
                              const std::counted_iterator<T1> &a1) {
  return operator+(std::forward<std::iter_difference_t<T1>>(a0), a1);
}
