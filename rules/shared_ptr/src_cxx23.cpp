#include <algorithm>
#include <barrier>
#include <chrono>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
std::shared_ptr<T1> f1(const T2 &a0, const std::remove_extent_t<T1> &a1) {
  return std::allocate_shared<T1, T2>(a0, a1);
}

template <typename T1, typename T2> std::shared_ptr<T1> f2(const T2 &a0) {
  return std::allocate_shared<T1, T2>(a0);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f3(const T2 &a0, std::size_t a1,
                       const std::remove_extent_t<T1> &a2) {
  return std::allocate_shared<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f4(const T2 &a0, std::size_t a1) {
  return std::allocate_shared<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename... T3>
std::shared_ptr<T1> f5(const T2 &a0, T3 &&...a1) {
  return std::allocate_shared<T1, T2, T3...>(a0, std::forward<T3>(a1)...);
}

template <typename T1, typename T2> std::shared_ptr<T1> f6(const T2 &a0) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f7(const T2 &a0, std::size_t a1) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f8(std::shared_ptr<T2> &&a0) {
  return std::const_pointer_cast<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::shared_ptr<T1> f9(const std::shared_ptr<T2> &a0) {
  return std::const_pointer_cast<T1, T2>(a0);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f10(std::shared_ptr<T2> &&a0) {
  return std::dynamic_pointer_cast<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::shared_ptr<T1> f11(const std::shared_ptr<T2> &a0) {
  return std::dynamic_pointer_cast<T1, T2>(a0);
}

template <typename T1, typename T2> T1 *f12(const std::shared_ptr<T2> &a0) {
  return std::get_deleter<T1, T2>(a0);
}

template <typename T1, typename T2, std::_Lock_policy T3>
T1 *f13(const std::__shared_ptr<T2, T3> &a0) {
  return std::get_deleter<T1, T2, T3>(a0);
}

template <typename T1>
std::shared_ptr<T1> f14(const std::remove_extent_t<T1> &a0) {
  return std::make_shared<T1>(a0);
}

template <typename T1> std::shared_ptr<T1> f15() {
  return std::make_shared<T1>();
}

template <typename T1>
std::shared_ptr<T1> f16(std::size_t a0, const std::remove_extent_t<T1> &a1) {
  return std::make_shared<T1>(a0, a1);
}

template <typename T1> std::shared_ptr<T1> f17(std::size_t a0) {
  return std::make_shared<T1>(a0);
}

template <typename T1, typename... T2> std::shared_ptr<T1> f18(T2 &&...a0) {
  return std::make_shared<T1, T2...>(std::forward<T2>(a0)...);
}

template <typename T1> std::shared_ptr<T1> f19() {
  return std::make_shared_for_overwrite<T1>();
}

template <typename T1> std::shared_ptr<T1> f20(std::size_t a0) {
  return std::make_shared_for_overwrite<T1>(a0);
}

template <typename T1, typename T2, typename T3, std::_Lock_policy T4>
std::basic_ostream<T1, T2> &f21(std::basic_ostream<T1, T2> &a0,
                                const std::__shared_ptr<T3, T4> &a1) {
  return std::operator<< <T1, T2, T3, T4>(a0, a1);
}

template <typename T1>
std::strong_ordering f22(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return std::operator<=> <T1>(a0, a1);
}

template <typename T1, typename T2>
std::strong_ordering f23(const std::shared_ptr<T1> &a0,
                         const std::shared_ptr<T2> &a1) {
  return std::operator<=> <T1, T2>(a0, a1);
}

template <typename T1>
bool f24(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1, typename T2>
bool f25(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f26(std::shared_ptr<T2> &&a0) {
  return std::reinterpret_pointer_cast<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::shared_ptr<T1> f27(const std::shared_ptr<T2> &a0) {
  return std::reinterpret_pointer_cast<T1, T2>(a0);
}

template <typename T1> using t1 = std::shared_ptr<T1>;

template <typename T1> using t2 = typename std::shared_ptr<T1>::element_type;

template <typename T1> using t3 = typename std::shared_ptr<T1>::weak_type;

template <typename T1> std::shared_ptr<T1> f28() {
  return std::shared_ptr<T1>();
}

template <typename T1> std::shared_ptr<T1> f29(const std::shared_ptr<T1> &a0) {
  return std::shared_ptr<T1>(a0);
}

template <typename T1> std::shared_ptr<T1> f30(std::shared_ptr<T1> &&a0) {
  return std::shared_ptr<T1>(std::move(a0));
}

template <typename T1> std::shared_ptr<T1> f31(std::nullptr_t a0) {
  return std::shared_ptr<T1>(a0);
}

template <typename T1>
std::shared_ptr<T1> &f32(std::shared_ptr<T1> &a0,
                         const std::shared_ptr<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::shared_ptr<T1> &f33(std::shared_ptr<T1> &a0, std::shared_ptr<T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2> std::shared_ptr<T1> f34(T2 *a0) {
  return std::shared_ptr<T1>(a0);
}

template <typename T1, typename T2, typename T3>
std::shared_ptr<T1> f35(T2 *a0, T3 a1) {
  return std::shared_ptr<T1>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::shared_ptr<T1> f36(std::nullptr_t a0, T2 a1) {
  return std::shared_ptr<T1>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::shared_ptr<T1> f37(T2 *a0, T3 a1, T4 a2) {
  return std::shared_ptr<T1>(a0, std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
std::shared_ptr<T1> f38(std::nullptr_t a0, T2 a1, T3 a2) {
  return std::shared_ptr<T1>(a0, std::move(a1), std::move(a2));
}

template <typename T1, typename T2>
std::shared_ptr<T1> f39(const std::shared_ptr<T2> &a0,
                        typename std::shared_ptr<T1>::element_type *a1) {
  return std::shared_ptr<T1>(a0, a1);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f40(std::shared_ptr<T2> &&a0,
                        typename std::shared_ptr<T1>::element_type *a1) {
  return std::shared_ptr<T1>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f41(const std::shared_ptr<T2> &a0)
  requires(!(std::is_same<T2, T1>::value))
{
  return std::shared_ptr<T1>(a0);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f42(std::shared_ptr<T2> &&a0)
  requires(!(std::is_same<T2, T1>::value))
{
  return std::shared_ptr<T1>(std::move(a0));
}

template <typename T1, typename T2>
std::shared_ptr<T1> f43(const std::weak_ptr<T2> &a0) {
  return std::shared_ptr<T1>(a0);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f44(std::auto_ptr<T2> &&a0) {
  return std::shared_ptr<T1>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::shared_ptr<T1> f45(std::unique_ptr<T2, T3> &&a0) {
  return std::shared_ptr<T1>(std::move(a0));
}

template <typename T1, typename T2>
std::shared_ptr<T1> &f46(std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1)
  requires(!(std::is_same<T2, T1>::value))
{
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
std::shared_ptr<T1> &f47(std::shared_ptr<T1> &a0, std::auto_ptr<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1, typename T2>
std::shared_ptr<T1> &f48(std::shared_ptr<T1> &a0, std::shared_ptr<T2> &&a1)
  requires(!(std::is_same<T2, T1>::value))
{
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::shared_ptr<T1> &f49(std::shared_ptr<T1> &a0,
                         std::unique_ptr<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}

template <typename T1> void f50(std::shared_ptr<T1> &a0) { return a0.reset(); }

template <typename T1>
typename std::shared_ptr<T1>::element_type *f51(const std::shared_ptr<T1> &a0) {
  return a0.get();
}

template <typename T1> bool f52(const std::shared_ptr<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> bool f53(const std::shared_ptr<T1> &a0) {
  return a0.unique();
}

template <typename T1> long f54(const std::shared_ptr<T1> &a0) {
  return a0.use_count();
}

template <typename T1>
void f55(std::shared_ptr<T1> &a0, std::shared_ptr<T1> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2> void f56(std::shared_ptr<T1> &a0, T2 *a1) {
  return a0.template reset<T2>(a1);
}

template <typename T1, typename T2, typename T3>
void f57(std::shared_ptr<T1> &a0, T2 *a1, T3 a2) {
  return a0.template reset<T2, T3>(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
void f58(std::shared_ptr<T1> &a0, T2 *a1, T3 a2, T4 a3) {
  return a0.template reset<T2, T3, T4>(a1, std::move(a2), std::move(a3));
}

template <typename T1, typename T2>
bool f59(const std::shared_ptr<T1> &a0,
         const std::__shared_ptr<T2, (std::_Lock_policy)2U> &a1) {
  return a0.template owner_before<T2>(a1);
}

template <typename T1, typename T2>
bool f60(const std::shared_ptr<T1> &a0,
         const std::__weak_ptr<T2, (std::_Lock_policy)2U> &a1) {
  return a0.template owner_before<T2>(a1);
}

template <typename T1> T1 &f61(const std::shared_ptr<T1> &a0) {
  return a0.operator*();
}

template <typename T1> T1 *f62(const std::shared_ptr<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
T1 *f63(const std::shared_ptr<T1> &a0)
  requires(((std::is_array<T1>::value) == false) &&
           ((std::is_void<T1>::value) == true))
{
  return a0.operator->();
}

template <typename T1>
typename std::shared_ptr<T1>::element_type &f64(const std::shared_ptr<T1> &a0,
                                                std::ptrdiff_t a1)
  requires(((std::is_array<T1>::value) == true) &&
           ((std::is_void<T1>::value) == false))
{
  return a0.operator[](a1);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f65(std::shared_ptr<T2> &&a0) {
  return std::static_pointer_cast<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::shared_ptr<T1> f66(const std::shared_ptr<T2> &a0) {
  return std::static_pointer_cast<T1, T2>(a0);
}

template <typename T1>
void f67(std::shared_ptr<T1> &a0, std::shared_ptr<T1> &a1) {
  return std::swap<T1>(a0, a1);
}

template <typename T1> void f68(std::weak_ptr<T1> &a0, std::weak_ptr<T1> &a1) {
  return std::swap<T1>(a0, a1);
}
