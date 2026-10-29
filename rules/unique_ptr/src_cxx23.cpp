#include <algorithm>
#include <barrier>
#include <utility>

template <typename T1> auto f1(std::size_t a0) {
  return std::make_unique<T1>(a0);
}

template <typename T1, typename... T2> std::unique_ptr<T1> f2(T2 &&...a0) {
  return std::make_unique<T1, T2...>(std::forward<T2>(a0)...);
}

template <typename T1> std::unique_ptr<T1> f3() {
  return std::make_unique_for_overwrite<T1>();
}

template <typename T1> auto f4(std::size_t a0) {
  return std::make_unique_for_overwrite<T1>(a0);
}

template <typename T1, typename T2>
bool f5(const std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f6(std::nullptr_t a0, const std::unique_ptr<T1, T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f7(const std::unique_ptr<T1, T2> &a0, const std::unique_ptr<T3, T4> &a1) {
  return std::operator< <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_ostream<T1, T2> &f8(std::basic_ostream<T1, T2> &a0,
                               const std::unique_ptr<T3, T4> &a1) {
  return std::operator<< <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2>
bool f9(const std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f10(std::nullptr_t a0, const std::unique_ptr<T1, T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f11(const std::unique_ptr<T1, T2> &a0, const std::unique_ptr<T3, T4> &a1) {
  return std::operator<= <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2>
std::compare_three_way_result_t<typename std::unique_ptr<T1, T2>::pointer>
f12(const std::unique_ptr<T1, T2> &a0, std::nullptr_t a1)
  requires(std::three_way_comparable<typename std::unique_ptr<T1, T2>::pointer>)
{
  return std::operator<=> <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::compare_three_way_result_t<typename std::unique_ptr<T1, T2>::pointer,
                                typename std::unique_ptr<T3, T4>::pointer>
f13(const std::unique_ptr<T1, T2> &a0, const std::unique_ptr<T3, T4> &a1)
  requires(
      std::three_way_comparable_with<typename std::unique_ptr<T1, T2>::pointer,
                                     typename std::unique_ptr<T3, T4>::pointer>)
{
  return std::operator<=> <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2>
bool f14(const std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f15(const std::unique_ptr<T1, T2> &a0, const std::unique_ptr<T3, T4> &a1) {
  return std::operator== <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2>
bool f16(const std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f17(std::nullptr_t a0, const std::unique_ptr<T1, T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f18(const std::unique_ptr<T1, T2> &a0, const std::unique_ptr<T3, T4> &a1) {
  return std::operator><T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2>
bool f19(const std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f20(std::nullptr_t a0, const std::unique_ptr<T1, T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f21(const std::unique_ptr<T1, T2> &a0, const std::unique_ptr<T3, T4> &a1) {
  return std::operator>= <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2>
typename std::enable_if<std::is_swappable_v<T2>>::type
f22(std::unique_ptr<T1, T2> &a0, std::unique_ptr<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2> using t1 = std::unique_ptr<T1[], T2>;

template <typename T1, typename T2>
using t2 = typename std::unique_ptr<T1[], T2>::pointer;

template <typename T1, typename T2>
using t3 = typename std::unique_ptr<T1[], T2>::element_type;

template <typename T1, typename T2>
using t4 = typename std::unique_ptr<T1[], T2>::deleter_type;

template <typename T1, typename T2>
std::unique_ptr<T1[], T2> f23(std::unique_ptr<T1[], T2> &&a0) {
  return std::unique_ptr<T1[], T2>(std::move(a0));
}

template <typename T1, typename T2>
std::unique_ptr<T1[], T2> &f24(std::unique_ptr<T1[], T2> &a0,
                               std::unique_ptr<T1[], T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::unique_ptr<T1[], T2> &f25(std::unique_ptr<T1[], T2> &a0,
                               std::nullptr_t a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
typename std::add_lvalue_reference<T1>::type
f26(const std::unique_ptr<T1[], T2> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
T1 *f27(const std::unique_ptr<T1[], T2> &a0) {
  return a0.get();
}

template <typename T1, typename T2> T2 &f28(std::unique_ptr<T1[], T2> &a0) {
  return a0.get_deleter();
}

template <typename T1, typename T2>
const T2 &f29(const std::unique_ptr<T1[], T2> &a0) {
  return a0.get_deleter();
}

template <typename T1, typename T2>
bool f30(const std::unique_ptr<T1[], T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2> T1 *f31(std::unique_ptr<T1[], T2> &a0) {
  return a0.release();
}

template <typename T1, typename T2>
void f32(std::unique_ptr<T1[], T2> &a0, std::nullptr_t a1) {
  return a0.reset(a1);
}

template <typename T1, typename T2>
void f33(std::unique_ptr<T1[], T2> &a0, std::unique_ptr<T1[], T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3 = T2,
          typename T4 = typename std::enable_if<std::integral_constant<
              bool, std::integral_constant<
                        bool, !std::is_pointer<T2>::value>::value &&
                        std::is_default_constructible<T2>::value>::value>::type>
std::unique_ptr<T1[], T2> f34() {
  return std::unique_ptr<T1[], T2>();
}

template <typename T1, typename T2, typename T3, typename T4 = T2,
          typename T5 = typename std::enable_if<std::integral_constant<
              bool, std::integral_constant<
                        bool, !std::is_pointer<T2>::value>::value &&
                        std::is_default_constructible<T2>::value>::value>::type>
std::unique_ptr<T1[], T2> f35(T3 a0)
  requires(!(std::is_same<T3, std::unique_ptr<T1[], T2> &&>::value) &&
           !(std::is_same<T3, std::nullptr_t>::value) &&
           !(std::is_same<T3, const std::unique_ptr<T1[], T2> &>::value))
{
  return std::unique_ptr<T1[], T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4 = T2>
std::unique_ptr<T1[], T2> f36(T3 a0, const T2 &a1) {
  return std::unique_ptr<T1[], T2>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4 = T2>
std::unique_ptr<T1[], T2>
f37(T3 a0,
    typename std::enable_if<!std::is_lvalue_reference<T4>::value, T4 &&>::type
        a1) {
  return std::unique_ptr<T1[], T2>(
      std::move(a0),
      std::forward<typename std::enable_if<!std::is_lvalue_reference<T4>::value,
                                           T4 &&>::type>(a1));
}

template <typename T1, typename T2, typename T3 = T2,
          typename T4 = typename std::enable_if<std::integral_constant<
              bool, std::integral_constant<
                        bool, !std::is_pointer<T2>::value>::value &&
                        std::is_default_constructible<T2>::value>::value>::type>
std::unique_ptr<T1[], T2> f38(std::nullptr_t a0) {
  return std::unique_ptr<T1[], T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unique_ptr<T1[], T2> f39(std::unique_ptr<T3, T4> &&a0)
  requires(!(std::is_same<T3, T1[]>::value && std::is_same<T4, T2>::value))
{
  return std::unique_ptr<T1[], T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::unique_ptr<T1[], T2> &f40(std::unique_ptr<T1[], T2> &a0,
                               std::unique_ptr<T3, T4> &&a1)
  requires(!(std::is_same<T3, T1[]>::value && std::is_same<T4, T2>::value))
{
  return a0.template operator= <T3, T4>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
void f41(std::unique_ptr<T1[], T2> &a0, T3 a1)
  requires(!(std::is_same<T3, std::nullptr_t>::value))
{
  return a0.template reset<T3>(std::move(a1));
}

template <typename T1, typename T2> using t5 = std::unique_ptr<T1, T2>;

template <typename T1, typename T2>
using t6 = typename std::unique_ptr<T1, T2>::pointer;

template <typename T1, typename T2>
using t7 = typename std::unique_ptr<T1, T2>::element_type;

template <typename T1, typename T2>
using t8 = typename std::unique_ptr<T1, T2>::deleter_type;

template <typename T1, typename T2>
std::unique_ptr<T1, T2> f42(std::unique_ptr<T1, T2> &&a0) {
  return std::unique_ptr<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::unique_ptr<T1, T2> &f43(std::unique_ptr<T1, T2> &a0,
                             std::unique_ptr<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::unique_ptr<T1, T2> &f44(std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
typename std::add_lvalue_reference<T1>::type
f45(const std::unique_ptr<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2> T1 *f46(const std::unique_ptr<T1, T2> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2> T1 *f47(const std::unique_ptr<T1, T2> &a0) {
  return a0.get();
}

template <typename T1, typename T2> T2 &f48(std::unique_ptr<T1, T2> &a0) {
  return a0.get_deleter();
}

template <typename T1, typename T2>
const T2 &f49(const std::unique_ptr<T1, T2> &a0) {
  return a0.get_deleter();
}

template <typename T1, typename T2>
bool f50(const std::unique_ptr<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2> T1 *f51(std::unique_ptr<T1, T2> &a0) {
  return a0.release();
}

template <typename T1, typename T2>
void f52(std::unique_ptr<T1, T2> &a0, T1 *a1) {
  return a0.reset(a1);
}

template <typename T1, typename T2>
void f53(std::unique_ptr<T1, T2> &a0, std::unique_ptr<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3 = T2,
          typename T4 = typename std::enable_if<std::integral_constant<
              bool, std::integral_constant<
                        bool, !std::is_pointer<T2>::value>::value &&
                        std::is_default_constructible<T2>::value>::value>::type>
std::unique_ptr<T1, T2> f54() {
  return std::unique_ptr<T1, T2>();
}

template <typename T1, typename T2, typename T3 = T2,
          typename T4 = typename std::enable_if<std::integral_constant<
              bool, std::integral_constant<
                        bool, !std::is_pointer<T2>::value>::value &&
                        std::is_default_constructible<T2>::value>::value>::type>
std::unique_ptr<T1, T2> f55(T1 *a0) {
  return std::unique_ptr<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3 = T2,
          typename T4 = typename std::enable_if<
              std::integral_constant<
                  bool, std::is_copy_constructible<T2>::value>::value,
              void>::type>
std::unique_ptr<T1, T2> f56(T1 *a0, const T2 &a1) {
  return std::unique_ptr<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3 = T2,
          typename T4 = typename std::enable_if<
              std::integral_constant<
                  bool, std::is_move_constructible<T3>::value>::value,
              void>::type>
std::unique_ptr<T1, T2>
f57(T1 *a0,
    typename std::enable_if<!std::is_lvalue_reference<T3>::value, T3 &&>::type
        a1) {
  return std::unique_ptr<T1, T2>(
      a0,
      std::forward<typename std::enable_if<!std::is_lvalue_reference<T3>::value,
                                           T3 &&>::type>(a1));
}

template <typename T1, typename T2, typename T3 = T2,
          typename T4 = typename std::enable_if<std::integral_constant<
              bool, std::integral_constant<
                        bool, !std::is_pointer<T2>::value>::value &&
                        std::is_default_constructible<T2>::value>::value>::type>
std::unique_ptr<T1, T2> f58(std::nullptr_t a0) {
  return std::unique_ptr<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unique_ptr<T1, T2> f59(std::unique_ptr<T3, T4> &&a0)
  requires(!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return std::unique_ptr<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::unique_ptr<T1, T2> f60(std::auto_ptr<T3> &&a0) {
  return std::unique_ptr<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::unique_ptr<T1, T2> &f61(std::unique_ptr<T1, T2> &a0,
                             std::unique_ptr<T3, T4> &&a1)
  requires(!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return a0.template operator= <T3, T4>(std::move(a1));
}
