#include <algorithm>
#include <barrier>
#include <chrono>
#include <utility>

template <typename T1, std::_Lock_policy T2>
using t1 = std::__shared_ptr<T1, T2>;

template <typename T1, std::_Lock_policy T2>
using t2 = typename std::__shared_ptr<T1, T2>::element_type;

template <typename T1, std::_Lock_policy T2>
using t3 = typename std::__shared_ptr<T1, T2>::weak_type;

template <typename T1, std::_Lock_policy T2> std::__shared_ptr<T1, T2> f1() {
  return std::__shared_ptr<T1, T2>();
}

template <typename T1, std::_Lock_policy T2>
std::__shared_ptr<T1, T2> f2(const std::__shared_ptr<T1, T2> &a0) {
  return std::__shared_ptr<T1, T2>(a0);
}

template <typename T1, std::_Lock_policy T2>
std::__shared_ptr<T1, T2> &f3(std::__shared_ptr<T1, T2> &a0,
                              const std::__shared_ptr<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, std::_Lock_policy T2>
std::__shared_ptr<T1, T2> f4(std::__shared_ptr<T1, T2> &&a0) {
  return std::__shared_ptr<T1, T2>(std::move(a0));
}

template <typename T1, std::_Lock_policy T2>
std::__shared_ptr<T1, T2> f5(std::nullptr_t a0) {
  return std::__shared_ptr<T1, T2>(a0);
}

template <typename T1, std::_Lock_policy T2>
std::__shared_ptr<T1, T2> &f6(std::__shared_ptr<T1, T2> &a0,
                              std::__shared_ptr<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, std::_Lock_policy T2>
void f7(std::__shared_ptr<T1, T2> &a0) {
  return a0.reset();
}

template <typename T1, std::_Lock_policy T2>
typename std::__shared_ptr<T1, T2>::element_type *
f8(const std::__shared_ptr<T1, T2> &a0) {
  return a0.get();
}

template <typename T1, std::_Lock_policy T2>
bool f9(const std::__shared_ptr<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, std::_Lock_policy T2>
bool f10(const std::__shared_ptr<T1, T2> &a0) {
  return a0.unique();
}

template <typename T1, std::_Lock_policy T2>
long f11(const std::__shared_ptr<T1, T2> &a0) {
  return a0.use_count();
}

template <typename T1, std::_Lock_policy T2>
void f12(std::__shared_ptr<T1, T2> &a0, std::__shared_ptr<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, std::_Lock_policy T2, typename T3>
std::__shared_ptr<T1, T2> f13(T3 *a0) {
  return std::__shared_ptr<T1, T2>(a0);
}

template <typename T1, std::_Lock_policy T2, typename T3, typename T4>
std::__shared_ptr<T1, T2> f14(T3 *a0, T4 a1) {
  return std::__shared_ptr<T1, T2>(a0, std::move(a1));
}

template <typename T1, std::_Lock_policy T2, typename T3, typename T4,
          typename T5>
std::__shared_ptr<T1, T2> f15(T3 *a0, T4 a1, T5 a2) {
  return std::__shared_ptr<T1, T2>(a0, std::move(a1), std::move(a2));
}

template <typename T1, std::_Lock_policy T2, typename T3>
std::__shared_ptr<T1, T2> f16(std::nullptr_t a0, T3 a1) {
  return std::__shared_ptr<T1, T2>(a0, std::move(a1));
}

template <typename T1, std::_Lock_policy T2, typename T3, typename T4>
std::__shared_ptr<T1, T2> f17(std::nullptr_t a0, T3 a1, T4 a2) {
  return std::__shared_ptr<T1, T2>(a0, std::move(a1), std::move(a2));
}

template <typename T1, std::_Lock_policy T2, typename T3>
std::__shared_ptr<T1, T2>
f18(const std::__shared_ptr<T3, T2> &a0,
    typename std::__shared_ptr<T1, T2>::element_type *a1) {
  return std::__shared_ptr<T1, T2>(a0, a1);
}

template <typename T1, std::_Lock_policy T2, typename T3>
std::__shared_ptr<T1, T2>
f19(std::__shared_ptr<T3, T2> &&a0,
    typename std::__shared_ptr<T1, T2>::element_type *a1) {
  return std::__shared_ptr<T1, T2>(std::move(a0), a1);
}

template <typename T1, std::_Lock_policy T2, typename T3>
std::__shared_ptr<T1, T2> f20(const std::__shared_ptr<T3, T2> &a0)
  requires(!(std::is_same<T3, T1>::value))
{
  return std::__shared_ptr<T1, T2>(a0);
}

template <typename T1, std::_Lock_policy T2, typename T3>
std::__shared_ptr<T1, T2> f21(std::__shared_ptr<T3, T2> &&a0)
  requires(!(std::is_same<T3, T1>::value))
{
  return std::__shared_ptr<T1, T2>(std::move(a0));
}

template <typename T1, std::_Lock_policy T2, typename T3>
std::__shared_ptr<T1, T2> f22(const std::__weak_ptr<T3, T2> &a0) {
  return std::__shared_ptr<T1, T2>(a0);
}

template <typename T1, std::_Lock_policy T2, typename T3, typename T4>
std::__shared_ptr<T1, T2> f23(std::unique_ptr<T3, T4> &&a0) {
  return std::__shared_ptr<T1, T2>(std::move(a0));
}

template <typename T1, std::_Lock_policy T2, typename T3>
std::__shared_ptr<T1, T2> f24(std::auto_ptr<T3> &&a0) {
  return std::__shared_ptr<T1, T2>(std::move(a0));
}

template <typename T1, std::_Lock_policy T2, typename T3>
std::__shared_ptr<T1, T2> &f25(std::__shared_ptr<T1, T2> &a0,
                               const std::__shared_ptr<T3, T2> &a1)
  requires(!(std::is_same<T3, T1>::value))
{
  return a0.template operator= <T3>(a1);
}

template <typename T1, std::_Lock_policy T2, typename T3>
std::__shared_ptr<T1, T2> &f26(std::__shared_ptr<T1, T2> &a0,
                               std::auto_ptr<T3> &&a1) {
  return a0.template operator= <T3>(std::move(a1));
}

template <typename T1, std::_Lock_policy T2, typename T3>
std::__shared_ptr<T1, T2> &f27(std::__shared_ptr<T1, T2> &a0,
                               std::__shared_ptr<T3, T2> &&a1)
  requires(!(std::is_same<T3, T1>::value))
{
  return a0.template operator= <T3>(std::move(a1));
}

template <typename T1, std::_Lock_policy T2, typename T3, typename T4>
std::__shared_ptr<T1, T2> &f28(std::__shared_ptr<T1, T2> &a0,
                               std::unique_ptr<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}

template <typename T1, std::_Lock_policy T2, typename T3>
void f29(std::__shared_ptr<T1, T2> &a0, T3 *a1) {
  return a0.template reset<T3>(a1);
}

template <typename T1, std::_Lock_policy T2, typename T3, typename T4>
void f30(std::__shared_ptr<T1, T2> &a0, T3 *a1, T4 a2) {
  return a0.template reset<T3, T4>(a1, std::move(a2));
}

template <typename T1, std::_Lock_policy T2, typename T3, typename T4,
          typename T5>
void f31(std::__shared_ptr<T1, T2> &a0, T3 *a1, T4 a2, T5 a3) {
  return a0.template reset<T3, T4, T5>(a1, std::move(a2), std::move(a3));
}

template <typename T1, std::_Lock_policy T2, typename T3>
bool f32(const std::__shared_ptr<T1, T2> &a0,
         const std::__shared_ptr<T3, T2> &a1) {
  return a0.template owner_before<T3>(a1);
}

template <typename T1, std::_Lock_policy T2, typename T3>
bool f33(const std::__shared_ptr<T1, T2> &a0,
         const std::__weak_ptr<T3, T2> &a1) {
  return a0.template owner_before<T3>(a1);
}

template <typename T1, std::_Lock_policy T2>
T1 &f34(const std::__shared_ptr<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, std::_Lock_policy T2>
T1 *f35(const std::__shared_ptr<T1, T2> &a0) {
  return a0.operator->();
}

template <typename T1, std::_Lock_policy T2>
T1 *f36(const std::__shared_ptr<T1, T2> &a0)
  requires(T2 == T2) && (std::is_array<T1>::value == false) &&
          (std::is_void<T1>::value == true)
{
  return a0.operator->();
}

template <typename T1, std::_Lock_policy T2>
typename std::__shared_ptr<T1, T2>::element_type &
f37(const std::__shared_ptr<T1, T2> &a0, std::ptrdiff_t a1)
  requires(T2 == T2) && (std::is_array<T1>::value == true) &&
          (std::is_void<T1>::value == false)
{
  return a0.operator[](a1);
}

template <typename T1, std::_Lock_policy T2> using t4 = std::__weak_ptr<T1, T2>;

template <typename T1, std::_Lock_policy T2>
using t5 = typename std::__weak_ptr<T1, T2>::element_type;

template <typename T1, std::_Lock_policy T2> std::__weak_ptr<T1, T2> f38() {
  return std::__weak_ptr<T1, T2>();
}

template <typename T1, std::_Lock_policy T2>
std::__weak_ptr<T1, T2> f39(const std::__weak_ptr<T1, T2> &a0) {
  return std::__weak_ptr<T1, T2>(a0);
}

template <typename T1, std::_Lock_policy T2>
std::__weak_ptr<T1, T2> f40(std::__weak_ptr<T1, T2> &&a0) {
  return std::__weak_ptr<T1, T2>(std::move(a0));
}

template <typename T1, std::_Lock_policy T2>
std::__weak_ptr<T1, T2> &f41(std::__weak_ptr<T1, T2> &a0,
                             const std::__weak_ptr<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, std::_Lock_policy T2>
std::__weak_ptr<T1, T2> &f42(std::__weak_ptr<T1, T2> &a0,
                             std::__weak_ptr<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, std::_Lock_policy T2>
std::__shared_ptr<T1, T2> f43(const std::__weak_ptr<T1, T2> &a0) {
  return a0.lock();
}

template <typename T1, std::_Lock_policy T2>
long f44(const std::__weak_ptr<T1, T2> &a0) {
  return a0.use_count();
}

template <typename T1, std::_Lock_policy T2>
bool f45(const std::__weak_ptr<T1, T2> &a0) {
  return a0.expired();
}

template <typename T1, std::_Lock_policy T2>
void f46(std::__weak_ptr<T1, T2> &a0) {
  return a0.reset();
}

template <typename T1, std::_Lock_policy T2>
void f47(std::__weak_ptr<T1, T2> &a0, std::__weak_ptr<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, std::_Lock_policy T2, typename T3>
std::__weak_ptr<T1, T2> f48(const std::__weak_ptr<T3, T2> &a0)
  requires(!(std::is_same<T3, T1>::value))
{
  return std::__weak_ptr<T1, T2>(a0);
}

template <typename T1, std::_Lock_policy T2, typename T3>
std::__weak_ptr<T1, T2> f49(const std::__shared_ptr<T3, T2> &a0) {
  return std::__weak_ptr<T1, T2>(a0);
}

template <typename T1, std::_Lock_policy T2, typename T3>
std::__weak_ptr<T1, T2> f50(std::__weak_ptr<T3, T2> &&a0)
  requires(!(std::is_same<T3, T1>::value))
{
  return std::__weak_ptr<T1, T2>(std::move(a0));
}

template <typename T1, std::_Lock_policy T2, typename T3>
std::__weak_ptr<T1, T2> &f51(std::__weak_ptr<T1, T2> &a0,
                             const std::__weak_ptr<T3, T2> &a1)
  requires(!(std::is_same<T3, T1>::value))
{
  return a0.template operator= <T3>(a1);
}

template <typename T1, std::_Lock_policy T2, typename T3>
std::__weak_ptr<T1, T2> &f52(std::__weak_ptr<T1, T2> &a0,
                             const std::__shared_ptr<T3, T2> &a1) {
  return a0.template operator= <T3>(a1);
}

template <typename T1, std::_Lock_policy T2, typename T3>
std::__weak_ptr<T1, T2> &f53(std::__weak_ptr<T1, T2> &a0,
                             std::__weak_ptr<T3, T2> &&a1)
  requires(!(std::is_same<T3, T1>::value))
{
  return a0.template operator= <T3>(std::move(a1));
}

template <typename T1, std::_Lock_policy T2, typename T3>
bool f54(const std::__weak_ptr<T1, T2> &a0,
         const std::__shared_ptr<T3, T2> &a1) {
  return a0.template owner_before<T3>(a1);
}

template <typename T1, std::_Lock_policy T2, typename T3>
bool f55(const std::__weak_ptr<T1, T2> &a0, const std::__weak_ptr<T3, T2> &a1) {
  return a0.template owner_before<T3>(a1);
}

using t6 = std::bad_weak_ptr;

const char *f56(const std::bad_weak_ptr &a0) { return a0.what(); }

std::bad_weak_ptr f57() { return std::bad_weak_ptr(); }

std::bad_weak_ptr f58(const std::bad_weak_ptr &a0) {
  return std::bad_weak_ptr(a0);
}

std::bad_weak_ptr &f59(std::bad_weak_ptr &a0, const std::bad_weak_ptr &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, std::_Lock_policy T3>
std::__shared_ptr<T1, T3> f60(const std::__shared_ptr<T2, T3> &a0) {
  return std::const_pointer_cast<T1, T2, T3>(a0);
}

template <typename T1, typename T2, std::_Lock_policy T3>
std::__shared_ptr<T1, T3> f61(const std::__shared_ptr<T2, T3> &a0) {
  return std::dynamic_pointer_cast<T1, T2, T3>(a0);
}

template <typename T1, std::_Lock_policy T2>
std::strong_ordering f62(const std::__shared_ptr<T1, T2> &a0,
                         std::nullptr_t a1) {
  return std::operator<=> <T1, T2>(a0, a1);
}

template <typename T1, typename T2, std::_Lock_policy T3>
std::strong_ordering f63(const std::__shared_ptr<T1, T3> &a0,
                         const std::__shared_ptr<T2, T3> &a1) {
  return std::operator<=> <T1, T2, T3>(a0, a1);
}

template <typename T1, std::_Lock_policy T2>
bool f64(const std::__shared_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2, std::_Lock_policy T3>
bool f65(const std::__shared_ptr<T1, T3> &a0,
         const std::__shared_ptr<T2, T3> &a1) {
  return std::operator== <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, std::_Lock_policy T3>
std::__shared_ptr<T1, T3> f66(const std::__shared_ptr<T2, T3> &a0) {
  return std::reinterpret_pointer_cast<T1, T2, T3>(a0);
}

template <typename T1, typename T2, std::_Lock_policy T3>
std::__shared_ptr<T1, T3> f67(const std::__shared_ptr<T2, T3> &a0) {
  return std::static_pointer_cast<T1, T2, T3>(a0);
}

template <typename T1, std::_Lock_policy T2>
void f68(std::__shared_ptr<T1, T2> &a0, std::__shared_ptr<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, std::_Lock_policy T2>
void f69(std::__weak_ptr<T1, T2> &a0, std::__weak_ptr<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
