#include <algorithm>
#include <utility>

template <typename T1> using t1 = std::basic_const_iterator<T1>;

template <typename T1>
using t2 = typename std::basic_const_iterator<T1>::iterator_concept;

template <typename T1>
using t3 = typename std::basic_const_iterator<T1>::value_type;

template <typename T1>
using t4 = typename std::basic_const_iterator<T1>::difference_type;

template <typename T1>
std::basic_const_iterator<T1> f1()
  requires(std::default_initializable<T1>)
{
  return std::basic_const_iterator<T1>();
}

template <typename T1> std::basic_const_iterator<T1> f2(T1 a0) {
  return std::basic_const_iterator<T1>(a0);
}

template <typename T1> const T1 &f3(const std::basic_const_iterator<T1> &a0) {
  return a0.base();
}

template <typename T1> T1 f4(std::basic_const_iterator<T1> &&a0) {
  return std::move(a0).base();
}

template <typename T1>
std::iter_const_reference_t<T1> f5(const std::basic_const_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
const auto *f6(const std::basic_const_iterator<T1> &a0)
  requires(std::is_lvalue_reference_v<std::iter_reference_t<T1>> &&
           std::same_as<std::remove_cvref_t<std::iter_reference_t<T1>>,
                        typename std::basic_const_iterator<T1>::value_type>)
{
  return a0.operator->();
}

template <typename T1>
std::basic_const_iterator<T1> &f7(std::basic_const_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1> void f8(std::basic_const_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
std::basic_const_iterator<T1> f9(std::basic_const_iterator<T1> &a0, int a1)
  requires(std::forward_iterator<T1>)
{
  return a0.operator++(a1);
}

template <typename T1>
std::basic_const_iterator<T1> &f10(std::basic_const_iterator<T1> &a0)
  requires(std::bidirectional_iterator<T1>)
{
  return a0.operator--();
}

template <typename T1>
std::basic_const_iterator<T1> f11(std::basic_const_iterator<T1> &a0, int a1)
  requires(std::bidirectional_iterator<T1>)
{
  return a0.operator--(a1);
}

template <typename T1>
std::basic_const_iterator<T1> &f12(std::basic_const_iterator<T1> &a0,
                                   std::iter_difference_t<T1> a1)
  requires(std::random_access_iterator<T1>)
{
  return a0.operator+=(a1);
}

template <typename T1>
std::basic_const_iterator<T1> &f13(std::basic_const_iterator<T1> &a0,
                                   std::iter_difference_t<T1> a1)
  requires(std::random_access_iterator<T1>)
{
  return a0.operator-=(a1);
}

template <typename T1>
std::iter_const_reference_t<T1> f14(const std::basic_const_iterator<T1> &a0,
                                    std::iter_difference_t<T1> a1)
  requires(std::random_access_iterator<T1>)
{
  return a0.operator[](a1);
}

template <typename T1>
bool f15(const std::basic_const_iterator<T1> &a0,
         const std::basic_const_iterator<T1> &a1)
  requires(std::random_access_iterator<T1>)
{
  return a0.operator<(a1);
}

template <typename T1>
bool f16(const std::basic_const_iterator<T1> &a0,
         const std::basic_const_iterator<T1> &a1)
  requires(std::random_access_iterator<T1>)
{
  return a0.operator>(a1);
}

template <typename T1>
bool f17(const std::basic_const_iterator<T1> &a0,
         const std::basic_const_iterator<T1> &a1)
  requires(std::random_access_iterator<T1>)
{
  return a0.operator<=(a1);
}

template <typename T1>
bool f18(const std::basic_const_iterator<T1> &a0,
         const std::basic_const_iterator<T1> &a1)
  requires(std::random_access_iterator<T1>)
{
  return a0.operator>=(a1);
}

template <typename T1>
auto f19(const std::basic_const_iterator<T1> &a0,
         const std::basic_const_iterator<T1> &a1)
  requires(std::random_access_iterator<T1> && std::three_way_comparable<T1>)
{
  return a0.operator<=>(a1);
}

template <typename T1, typename T2>
std::basic_const_iterator<T1> f20(std::basic_const_iterator<T2> a0)
  requires(std::convertible_to<T2, T1>)
{
  return std::basic_const_iterator<T1>(a0);
}

template <typename T1, typename T2>
std::basic_const_iterator<T1> f21(T2 &&a0)
  requires(std::convertible_to<T2, T1>) &&
          (std::__detail::__different_from<T2,
                                           std::basic_const_iterator<T1>>) &&
          (!(std::is_same<T2, T1>::value))
{
  return std::basic_const_iterator<T1>(std::forward<T2>(a0));
}

template <typename T1, typename T2>
bool f22(const std::basic_const_iterator<T1> &a0, const T2 &a1)
  requires(std::sentinel_for<T2, T1>)
{
  return a0.template operator== <T2>(a1);
}

template <typename T1, typename T2>
T2 f23(const std::basic_const_iterator<T1> &a0)
  requires(std::__detail::__constant_iterator<T2> &&
           std::convertible_to<T1, T2>) &&
          (std::__detail::__not_a_const_iterator<T2>)
{
  return static_cast<T2>(a0);
}

template <typename T1, typename T2>
T2 f24(std::basic_const_iterator<T1> &&a0)
  requires(std::__detail::__constant_iterator<T2> &&
           std::convertible_to<T1, T2>) &&
          (std::__detail::__not_a_const_iterator<T2>)
{
  return static_cast<T2>(std::move(a0));
}

template <typename T1, typename T2>
bool f25(const std::basic_const_iterator<T1> &a0, const T2 &a1)
  requires(std::random_access_iterator<T1> &&
           std::totally_ordered_with<T1, T2>) &&
          (std::__detail::__different_from<T2,
                                           std::basic_const_iterator<T1>>) &&
          (!(std::is_same<T2, std::basic_const_iterator<T1>>::value))
{
  return a0.template operator< <T2>(a1);
}

template <typename T1, typename T2>
bool f26(const std::basic_const_iterator<T1> &a0, const T2 &a1)
  requires(std::random_access_iterator<T1> &&
           std::totally_ordered_with<T1, T2>) &&
          (std::__detail::__different_from<T2,
                                           std::basic_const_iterator<T1>>) &&
          (!(std::is_same<T2, std::basic_const_iterator<T1>>::value))
{
  return a0.template operator><T2>(a1);
}

template <typename T1, typename T2>
bool f27(const std::basic_const_iterator<T1> &a0, const T2 &a1)
  requires(std::random_access_iterator<T1> &&
           std::totally_ordered_with<T1, T2>) &&
          (std::__detail::__different_from<T2,
                                           std::basic_const_iterator<T1>>) &&
          (!(std::is_same<T2, std::basic_const_iterator<T1>>::value))
{
  return a0.template operator<= <T2>(a1);
}

template <typename T1, typename T2>
bool f28(const std::basic_const_iterator<T1> &a0, const T2 &a1)
  requires(std::random_access_iterator<T1> &&
           std::totally_ordered_with<T1, T2>) &&
          (std::__detail::__different_from<T2,
                                           std::basic_const_iterator<T1>>) &&
          (!(std::is_same<T2, std::basic_const_iterator<T1>>::value))
{
  return a0.template operator>= <T2>(a1);
}

template <typename T1, typename T2>
auto f29(const std::basic_const_iterator<T1> &a0, const T2 &a1)
  requires(std::random_access_iterator<T1> &&
           std::totally_ordered_with<T1, T2> &&
           std::three_way_comparable_with<T1, T2>) &&
          (std::__detail::__different_from<T2,
                                           std::basic_const_iterator<T1>>) &&
          (!(std::is_same<T2, std::basic_const_iterator<T1>>::value))
{
  return a0.template operator<=> <T2>(a1);
}

template <typename T1, typename T2>
std::iter_difference_t<T1> f30(const std::basic_const_iterator<T1> &a0,
                               const T2 &a1)
  requires(std::sized_sentinel_for<T2, T1>)
{
  return a0.template operator- <T2>(a1);
}

template <typename T1>
std::common_reference_t<const std::iter_value_t<T1> &&,
                        std::iter_rvalue_reference_t<T1>>
f31(const std::basic_const_iterator<T1> &a0) {
  return iter_move(a0);
}

template <typename T1, typename T2, typename T3>
std::iter_difference_t<T1> f32(const T2 &a0,
                               const std::basic_const_iterator<T3> &a1)
  requires(std::sized_sentinel_for<T2, T1>) &&
          (std::__detail::__not_a_const_iterator<T2>) && (std::same_as<T3, T1>)
{
  return operator-(a0, a1);
}

template <typename T1>
std::basic_const_iterator<T1> f33(const std::basic_const_iterator<T1> &a0,
                                  std::iter_difference_t<T1> a1) {
  return operator-(a0, a1);
}

template <typename T1>
std::basic_const_iterator<T1> f34(std::iter_difference_t<T1> a0,
                                  const std::basic_const_iterator<T1> &a1) {
  return operator+(a0, a1);
}

template <typename T1>
std::basic_const_iterator<T1> f35(const std::basic_const_iterator<T1> &a0,
                                  std::iter_difference_t<T1> a1) {
  return operator+(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f36(const T2 &a0, const std::basic_const_iterator<T3> &a1)
  requires(std::random_access_iterator<T1> &&
           std::totally_ordered_with<T1, T2>) &&
          (std::__detail::__not_a_const_iterator<T2>) && (std::same_as<T3, T1>)
{
  return operator>=(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f37(const T2 &a0, const std::basic_const_iterator<T3> &a1)
  requires(std::random_access_iterator<T1> &&
           std::totally_ordered_with<T1, T2>) &&
          (std::__detail::__not_a_const_iterator<T2>) && (std::same_as<T3, T1>)
{
  return operator<=(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f38(const T2 &a0, const std::basic_const_iterator<T3> &a1)
  requires(std::random_access_iterator<T1> &&
           std::totally_ordered_with<T1, T2>) &&
          (std::__detail::__not_a_const_iterator<T2>) && (std::same_as<T3, T1>)
{
  return operator>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f39(const T2 &a0, const std::basic_const_iterator<T3> &a1)
  requires(std::random_access_iterator<T1> &&
           std::totally_ordered_with<T1, T2>) &&
          (std::__detail::__not_a_const_iterator<T2>) && (std::same_as<T3, T1>)
{
  return operator<(a0, a1);
}
