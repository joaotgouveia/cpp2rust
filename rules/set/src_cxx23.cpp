#include <algorithm>
#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
typename std::multiset<T1, T2, T3>::size_type f1(std::multiset<T1, T2, T3> &a0,
                                                 T4 a1) {
  return std::erase_if(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::set<T1, T2, T3>::size_type f2(std::set<T1, T2, T3> &a0, T4 a1) {
  return std::erase_if(a0, std::move(a1));
}

template <typename T1, typename T2> using t1 = std::pmr::multiset<T1, T2>;

template <typename T1, typename T2> using t2 = std::pmr::set<T1, T2>;

template <typename T1, typename T2, typename T3>
using t3 = std::set<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t4 = typename std::set<T1, T2, T3>::key_type;

template <typename T1, typename T2, typename T3>
using t5 = typename std::set<T1, T2, T3>::value_type;

template <typename T1, typename T2, typename T3>
using t6 = typename std::set<T1, T2, T3>::key_compare;

template <typename T1, typename T2, typename T3>
using t7 = typename std::set<T1, T2, T3>::value_compare;

template <typename T1, typename T2, typename T3>
using t8 = typename std::set<T1, T2, T3>::allocator_type;

template <typename T1, typename T2, typename T3>
using t9 = typename std::set<T1, T2, T3>::pointer;

template <typename T1, typename T2, typename T3>
using t10 = typename std::set<T1, T2, T3>::const_pointer;

template <typename T1, typename T2, typename T3>
using t11 = typename std::set<T1, T2, T3>::reference;

template <typename T1, typename T2, typename T3>
using t12 = typename std::set<T1, T2, T3>::const_reference;

template <typename T1, typename T2, typename T3>
using t13 = typename std::set<T1, T2, T3>::iterator;

template <typename T1, typename T2, typename T3>
using t14 = typename std::set<T1, T2, T3>::const_iterator;

template <typename T1, typename T2, typename T3>
using t15 = typename std::set<T1, T2, T3>::reverse_iterator;

template <typename T1, typename T2, typename T3>
using t16 = typename std::set<T1, T2, T3>::const_reverse_iterator;

template <typename T1, typename T2, typename T3>
using t17 = typename std::set<T1, T2, T3>::size_type;

template <typename T1, typename T2, typename T3>
using t18 = typename std::set<T1, T2, T3>::difference_type;

template <typename T1, typename T2, typename T3>
using t19 = typename std::set<T1, T2, T3>::node_type;

template <typename T1, typename T2, typename T3>
using t20 = typename std::set<T1, T2, T3>::insert_return_type;

template <typename T1, typename T2, typename T3> std::set<T1, T2, T3> f3() {
  return std::set<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3> f4(const T2 &a0, const T3 &a1) {
  return std::set<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3> f5(const std::set<T1, T2, T3> &a0) {
  return std::set<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3> f6(std::set<T1, T2, T3> &&a0) {
  return std::set<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3> f7(std::initializer_list<T1> a0, const T2 &a1,
                        const T3 &a2) {
  return std::set<T1, T2, T3>(std::forward<std::initializer_list<T1>>(a0), a1,
                              a2);
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3> f8(const T3 &a0) {
  return std::set<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3> f9(const std::set<T1, T2, T3> &a0, const T3 &a1) {
  return std::set<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3> f10(std::set<T1, T2, T3> &&a0, const T3 &a1) {
  return std::set<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3> f11(std::initializer_list<T1> a0, const T3 &a1) {
  return std::set<T1, T2, T3>(std::forward<std::initializer_list<T1>>(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3> &f12(std::set<T1, T2, T3> &a0,
                          const std::set<T1, T2, T3> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3> &f13(std::set<T1, T2, T3> &a0, std::set<T1, T2, T3> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3> &f14(std::set<T1, T2, T3> &a0,
                          std::initializer_list<T1> a1) {
  return a0.operator=(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2, typename T3>
T2 f15(const std::set<T1, T2, T3> &a0) {
  return a0.key_comp();
}

template <typename T1, typename T2, typename T3>
T2 f16(const std::set<T1, T2, T3> &a0) {
  return a0.value_comp();
}

template <typename T1, typename T2, typename T3>
T3 f17(const std::set<T1, T2, T3> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator f18(const std::set<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator f19(const std::set<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::reverse_iterator
f20(const std::set<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::reverse_iterator
f21(const std::set<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator f22(const std::set<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator f23(const std::set<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::reverse_iterator
f24(const std::set<T1, T2, T3> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::reverse_iterator
f25(const std::set<T1, T2, T3> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3>
bool f26(const std::set<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::size_type f27(const std::set<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::size_type f28(const std::set<T1, T2, T3> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3>
void f29(std::set<T1, T2, T3> &a0, std::set<T1, T2, T3> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::set<T1, T2, T3>::iterator, bool>
f30(std::set<T1, T2, T3> &a0, const T1 &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::set<T1, T2, T3>::iterator, bool>
f31(std::set<T1, T2, T3> &a0, T1 &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator
f32(std::set<T1, T2, T3> &a0, typename std::set<T1, T2, T3>::const_iterator a1,
    const T1 &a2) {
  return a0.insert(
      std::forward<typename std::set<T1, T2, T3>::const_iterator>(a1), a2);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator
f33(std::set<T1, T2, T3> &a0, typename std::set<T1, T2, T3>::const_iterator a1,
    T1 &&a2) {
  return a0.insert(
      std::forward<typename std::set<T1, T2, T3>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f34(std::set<T1, T2, T3> &a0, std::initializer_list<T1> a1) {
  return a0.insert(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::node_type
f35(std::set<T1, T2, T3> &a0,
    typename std::set<T1, T2, T3>::const_iterator a1) {
  return a0.extract(
      std::forward<typename std::set<T1, T2, T3>::const_iterator>(a1));
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::node_type f36(std::set<T1, T2, T3> &a0,
                                             const T1 &a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::insert_return_type
f37(std::set<T1, T2, T3> &a0, typename std::set<T1, T2, T3>::node_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator
f38(std::set<T1, T2, T3> &a0, typename std::set<T1, T2, T3>::const_iterator a1,
    typename std::set<T1, T2, T3>::node_type &&a2) {
  return a0.insert(
      std::forward<typename std::set<T1, T2, T3>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator
f39(std::set<T1, T2, T3> &a0,
    typename std::set<T1, T2, T3>::const_iterator a1) {
  return a0.erase(
      std::forward<typename std::set<T1, T2, T3>::const_iterator>(a1));
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::size_type f40(std::set<T1, T2, T3> &a0,
                                             const T1 &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator
f41(std::set<T1, T2, T3> &a0, typename std::set<T1, T2, T3>::const_iterator a1,
    typename std::set<T1, T2, T3>::const_iterator a2) {
  return a0.erase(
      std::forward<typename std::set<T1, T2, T3>::const_iterator>(a1),
      std::forward<typename std::set<T1, T2, T3>::const_iterator>(a2));
}

template <typename T1, typename T2, typename T3>
void f42(std::set<T1, T2, T3> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::size_type f43(const std::set<T1, T2, T3> &a0,
                                             const T1 &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3>
bool f44(const std::set<T1, T2, T3> &a0, const T1 &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator f45(std::set<T1, T2, T3> &a0,
                                            const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_iterator
f46(const std::set<T1, T2, T3> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator f47(std::set<T1, T2, T3> &a0,
                                            const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_iterator
f48(const std::set<T1, T2, T3> &a0, const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator f49(std::set<T1, T2, T3> &a0,
                                            const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_iterator
f50(const std::set<T1, T2, T3> &a0, const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::set<T1, T2, T3>::iterator,
          typename std::set<T1, T2, T3>::iterator>
f51(std::set<T1, T2, T3> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::set<T1, T2, T3>::const_iterator,
          typename std::set<T1, T2, T3>::const_iterator>
f52(const std::set<T1, T2, T3> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::set<T1, T2, T3> f53(T4 a0, T4 a1) {
  return std::set<T1, T2, T3>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::set<T1, T2, T3> f54(T4 a0, T4 a1, const T2 &a2, const T3 &a3) {
  return std::set<T1, T2, T3>(std::move(a0), std::move(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::set<T1, T2, T3> f55(T4 a0, T4 a1, const T3 &a2) {
  return std::set<T1, T2, T3>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::set<T1, T2, T3> f56(std::from_range_t a0, T4 &&a1, const T2 &a2,
                         const T3 &a3) {
  return std::set<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::set<T1, T2, T3> f57(std::from_range_t a0, T4 &&a1, const T3 &a2) {
  return std::set<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename... T4>
std::pair<typename std::set<T1, T2, T3>::iterator, bool>
f58(std::set<T1, T2, T3> &a0, T4 &&...a1) {
  return a0.template emplace<T4...>(std::forward<T4>(a1)...);
}

template <typename T1, typename T2, typename T3, typename... T4>
typename std::set<T1, T2, T3>::iterator
f59(std::set<T1, T2, T3> &a0, typename std::set<T1, T2, T3>::const_iterator a1,
    T4 &&...a2) {
  return a0.template emplace_hint<T4...>(
      std::forward<typename std::set<T1, T2, T3>::const_iterator>(a1),
      std::forward<T4>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4>
void f60(std::set<T1, T2, T3> &a0, T4 a1, T4 a2) {
  return a0.template insert<T4>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
void f61(std::set<T1, T2, T3> &a0, T4 &&a1) {
  return a0.template insert_range<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
void f62(std::set<T1, T2, T3> &a0, std::set<T1, T4, T3> &a1) {
  return a0.template merge<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f63(std::set<T1, T2, T3> &a0, std::set<T1, T4, T3> &&a1) {
  return a0.template merge<T4>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
void f64(std::set<T1, T2, T3> &a0, std::multiset<T1, T4, T3> &a1) {
  return a0.template merge<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f65(std::set<T1, T2, T3> &a0, std::multiset<T1, T4, T3> &&a1) {
  return a0.template merge<T4>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
auto f66(const std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template count<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f67(const std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template contains<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f68(std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f69(const std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f70(std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f71(const std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f72(std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f73(const std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f74(std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template equal_range<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f75(const std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template equal_range<T4>(a1);
}

template <typename T1, typename T2, typename T3>
using t21 = typename std::set<T1, T2, T3>::iterator::value_type;

template <typename T1, typename T2, typename T3>
using t22 = typename std::set<T1, T2, T3>::iterator::reference;

template <typename T1, typename T2, typename T3>
using t23 = typename std::set<T1, T2, T3>::iterator::pointer;

template <typename T1, typename T2, typename T3>
using t24 = typename std::set<T1, T2, T3>::iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t25 = typename std::set<T1, T2, T3>::iterator::difference_type;
