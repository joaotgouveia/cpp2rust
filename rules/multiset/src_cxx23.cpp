#include <algorithm>
#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::multiset<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::multiset<T1, T2, T3>::key_type;

template <typename T1, typename T2, typename T3>
using t3 = typename std::multiset<T1, T2, T3>::value_type;

template <typename T1, typename T2, typename T3>
using t4 = typename std::multiset<T1, T2, T3>::key_compare;

template <typename T1, typename T2, typename T3>
using t5 = typename std::multiset<T1, T2, T3>::value_compare;

template <typename T1, typename T2, typename T3>
using t6 = typename std::multiset<T1, T2, T3>::allocator_type;

template <typename T1, typename T2, typename T3>
using t7 = typename std::multiset<T1, T2, T3>::pointer;

template <typename T1, typename T2, typename T3>
using t8 = typename std::multiset<T1, T2, T3>::const_pointer;

template <typename T1, typename T2, typename T3>
using t9 = typename std::multiset<T1, T2, T3>::reference;

template <typename T1, typename T2, typename T3>
using t10 = typename std::multiset<T1, T2, T3>::const_reference;

template <typename T1, typename T2, typename T3>
using t11 = typename std::multiset<T1, T2, T3>::iterator;

template <typename T1, typename T2, typename T3>
using t12 = typename std::multiset<T1, T2, T3>::const_iterator;

template <typename T1, typename T2, typename T3>
using t13 = typename std::multiset<T1, T2, T3>::reverse_iterator;

template <typename T1, typename T2, typename T3>
using t14 = typename std::multiset<T1, T2, T3>::const_reverse_iterator;

template <typename T1, typename T2, typename T3>
using t15 = typename std::multiset<T1, T2, T3>::size_type;

template <typename T1, typename T2, typename T3>
using t16 = typename std::multiset<T1, T2, T3>::difference_type;

template <typename T1, typename T2, typename T3>
using t17 = typename std::multiset<T1, T2, T3>::node_type;

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3> f1() {
  return std::multiset<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3> f2(const T2 &a0, const T3 &a1) {
  return std::multiset<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3> f3(const std::multiset<T1, T2, T3> &a0) {
  return std::multiset<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3> f4(std::multiset<T1, T2, T3> &&a0) {
  return std::multiset<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3> f5(std::initializer_list<T1> a0, const T2 &a1,
                             const T3 &a2) {
  return std::multiset<T1, T2, T3>(std::forward<std::initializer_list<T1>>(a0),
                                   a1, a2);
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3> f6(const T3 &a0) {
  return std::multiset<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3> f7(const std::multiset<T1, T2, T3> &a0,
                             const T3 &a1) {
  return std::multiset<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3> f8(std::multiset<T1, T2, T3> &&a0, const T3 &a1) {
  return std::multiset<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3> f9(std::initializer_list<T1> a0, const T3 &a1) {
  return std::multiset<T1, T2, T3>(std::forward<std::initializer_list<T1>>(a0),
                                   a1);
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3> &f10(std::multiset<T1, T2, T3> &a0,
                               const std::multiset<T1, T2, T3> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3> &f11(std::multiset<T1, T2, T3> &a0,
                               std::multiset<T1, T2, T3> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3> &f12(std::multiset<T1, T2, T3> &a0,
                               std::initializer_list<T1> a1) {
  return a0.operator=(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2, typename T3>
T2 f13(const std::multiset<T1, T2, T3> &a0) {
  return a0.key_comp();
}

template <typename T1, typename T2, typename T3>
T2 f14(const std::multiset<T1, T2, T3> &a0) {
  return a0.value_comp();
}

template <typename T1, typename T2, typename T3>
T3 f15(const std::multiset<T1, T2, T3> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f16(const std::multiset<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f17(const std::multiset<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::reverse_iterator
f18(const std::multiset<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::reverse_iterator
f19(const std::multiset<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f20(const std::multiset<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f21(const std::multiset<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::reverse_iterator
f22(const std::multiset<T1, T2, T3> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::reverse_iterator
f23(const std::multiset<T1, T2, T3> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3>
bool f24(const std::multiset<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::size_type
f25(const std::multiset<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::size_type
f26(const std::multiset<T1, T2, T3> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3>
void f27(std::multiset<T1, T2, T3> &a0, std::multiset<T1, T2, T3> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator f28(std::multiset<T1, T2, T3> &a0,
                                                 const T1 &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator f29(std::multiset<T1, T2, T3> &a0,
                                                 T1 &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f30(std::multiset<T1, T2, T3> &a0,
    typename std::multiset<T1, T2, T3>::const_iterator a1, const T1 &a2) {
  return a0.insert(
      std::forward<typename std::multiset<T1, T2, T3>::const_iterator>(a1), a2);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f31(std::multiset<T1, T2, T3> &a0,
    typename std::multiset<T1, T2, T3>::const_iterator a1, T1 &&a2) {
  return a0.insert(
      std::forward<typename std::multiset<T1, T2, T3>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f32(std::multiset<T1, T2, T3> &a0, std::initializer_list<T1> a1) {
  return a0.insert(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::node_type
f33(std::multiset<T1, T2, T3> &a0,
    typename std::multiset<T1, T2, T3>::const_iterator a1) {
  return a0.extract(
      std::forward<typename std::multiset<T1, T2, T3>::const_iterator>(a1));
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::node_type f34(std::multiset<T1, T2, T3> &a0,
                                                  const T1 &a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f35(std::multiset<T1, T2, T3> &a0,
    typename std::multiset<T1, T2, T3>::node_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f36(std::multiset<T1, T2, T3> &a0,
    typename std::multiset<T1, T2, T3>::const_iterator a1,
    typename std::multiset<T1, T2, T3>::node_type &&a2) {
  return a0.insert(
      std::forward<typename std::multiset<T1, T2, T3>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f37(std::multiset<T1, T2, T3> &a0,
    typename std::multiset<T1, T2, T3>::const_iterator a1) {
  return a0.erase(
      std::forward<typename std::multiset<T1, T2, T3>::const_iterator>(a1));
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::size_type f38(std::multiset<T1, T2, T3> &a0,
                                                  const T1 &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f39(std::multiset<T1, T2, T3> &a0,
    typename std::multiset<T1, T2, T3>::const_iterator a1,
    typename std::multiset<T1, T2, T3>::const_iterator a2) {
  return a0.erase(
      std::forward<typename std::multiset<T1, T2, T3>::const_iterator>(a1),
      std::forward<typename std::multiset<T1, T2, T3>::const_iterator>(a2));
}

template <typename T1, typename T2, typename T3>
void f40(std::multiset<T1, T2, T3> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::size_type
f41(const std::multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3>
bool f42(const std::multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator f43(std::multiset<T1, T2, T3> &a0,
                                                 const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::const_iterator
f44(const std::multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator f45(std::multiset<T1, T2, T3> &a0,
                                                 const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::const_iterator
f46(const std::multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator f47(std::multiset<T1, T2, T3> &a0,
                                                 const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::const_iterator
f48(const std::multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::multiset<T1, T2, T3>::iterator,
          typename std::multiset<T1, T2, T3>::iterator>
f49(std::multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::multiset<T1, T2, T3>::const_iterator,
          typename std::multiset<T1, T2, T3>::const_iterator>
f50(const std::multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multiset<T1, T2, T3> f51(T4 a0, T4 a1) {
  return std::multiset<T1, T2, T3>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::multiset<T1, T2, T3> f52(T4 a0, T4 a1, const T2 &a2, const T3 &a3) {
  return std::multiset<T1, T2, T3>(std::move(a0), std::move(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multiset<T1, T2, T3> f53(T4 a0, T4 a1, const T3 &a2) {
  return std::multiset<T1, T2, T3>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multiset<T1, T2, T3> f54(std::from_range_t a0, T4 &&a1, const T2 &a2,
                              const T3 &a3) {
  return std::multiset<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multiset<T1, T2, T3> f55(std::from_range_t a0, T4 &&a1, const T3 &a2) {
  return std::multiset<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename... T4>
typename std::multiset<T1, T2, T3>::iterator f56(std::multiset<T1, T2, T3> &a0,
                                                 T4 &&...a1) {
  return a0.template emplace<T4...>(std::forward<T4>(a1)...);
}

template <typename T1, typename T2, typename T3, typename... T4>
typename std::multiset<T1, T2, T3>::iterator
f57(std::multiset<T1, T2, T3> &a0,
    typename std::multiset<T1, T2, T3>::const_iterator a1, T4 &&...a2) {
  return a0.template emplace_hint<T4...>(
      std::forward<typename std::multiset<T1, T2, T3>::const_iterator>(a1),
      std::forward<T4>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4>
void f58(std::multiset<T1, T2, T3> &a0, T4 a1, T4 a2) {
  return a0.template insert<T4>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
void f59(std::multiset<T1, T2, T3> &a0, T4 &&a1) {
  return a0.template insert_range<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
void f60(std::multiset<T1, T2, T3> &a0, std::multiset<T1, T4, T3> &a1) {
  return a0.template merge<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f61(std::multiset<T1, T2, T3> &a0, std::multiset<T1, T4, T3> &&a1) {
  return a0.template merge<T4>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
void f62(std::multiset<T1, T2, T3> &a0, std::set<T1, T4, T3> &a1) {
  return a0.template merge<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f63(std::multiset<T1, T2, T3> &a0, std::set<T1, T4, T3> &&a1) {
  return a0.template merge<T4>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
auto f64(const std::multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(
      !(std::is_same<T4, typename std::multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template count<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f65(const std::multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(
      !(std::is_same<T4, typename std::multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template contains<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f66(std::multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(
      !(std::is_same<T4, typename std::multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f67(const std::multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(
      !(std::is_same<T4, typename std::multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f68(std::multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(
      !(std::is_same<T4, typename std::multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f69(const std::multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(
      !(std::is_same<T4, typename std::multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f70(std::multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(
      !(std::is_same<T4, typename std::multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f71(const std::multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(
      !(std::is_same<T4, typename std::multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f72(std::multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(
      !(std::is_same<T4, typename std::multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template equal_range<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f73(const std::multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(
      !(std::is_same<T4, typename std::multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template equal_range<T4>(a1);
}
