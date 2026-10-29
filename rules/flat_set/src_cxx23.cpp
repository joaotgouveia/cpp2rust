#include <algorithm>
#include <flat_set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_multiset<T1, T2, T3>::size_type
f1(std::flat_multiset<T1, T2, T3> &a0, T4 a1) {
  return std::erase_if(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::size_type f2(std::flat_set<T1, T2, T3> &a0,
                                                 T4 a1) {
  return std::erase_if(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3>
using t1 = std::flat_set<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::flat_set<T1, T2, T3>::key_type;

template <typename T1, typename T2, typename T3>
using t3 = typename std::flat_set<T1, T2, T3>::value_type;

template <typename T1, typename T2, typename T3>
using t4 = typename std::flat_set<T1, T2, T3>::key_compare;

template <typename T1, typename T2, typename T3>
using t5 = typename std::flat_set<T1, T2, T3>::reference;

template <typename T1, typename T2, typename T3>
using t6 = typename std::flat_set<T1, T2, T3>::const_reference;

template <typename T1, typename T2, typename T3>
using t7 = typename std::flat_set<T1, T2, T3>::size_type;

template <typename T1, typename T2, typename T3>
using t8 = typename std::flat_set<T1, T2, T3>::difference_type;

template <typename T1, typename T2, typename T3>
using t9 = typename std::flat_set<T1, T2, T3>::iterator;

template <typename T1, typename T2, typename T3>
using t10 = typename std::flat_set<T1, T2, T3>::const_iterator;

template <typename T1, typename T2, typename T3>
using t11 = typename std::flat_set<T1, T2, T3>::reverse_iterator;

template <typename T1, typename T2, typename T3>
using t12 = typename std::flat_set<T1, T2, T3>::const_reverse_iterator;

template <typename T1, typename T2, typename T3>
using t13 = typename std::flat_set<T1, T2, T3>::container_type;

template <typename T1, typename T2, typename T3>
using t14 = typename std::flat_set<T1, T2, T3>::value_compare;

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f3(const std::flat_set<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f4(const std::flat_set<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_reverse_iterator
f5(const std::flat_set<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_reverse_iterator
f6(const std::flat_set<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f7(const std::flat_set<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f8(const std::flat_set<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_reverse_iterator
f9(const std::flat_set<T1, T2, T3> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_reverse_iterator
f10(const std::flat_set<T1, T2, T3> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3>
bool f11(const std::flat_set<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::size_type
f12(const std::flat_set<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::size_type
f13(const std::flat_set<T1, T2, T3> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3>
std::pair<typename T3::const_iterator, bool> f14(std::flat_set<T1, T2, T3> &a0,
                                                 T1 const &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename T3::const_iterator, bool> f15(std::flat_set<T1, T2, T3> &a0,
                                                 T1 &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator
f16(std::flat_set<T1, T2, T3> &a0,
    typename std::flat_set<T1, T2, T3>::const_iterator a1, T1 const &a2) {
  return a0.insert(
      std::forward<typename std::flat_set<T1, T2, T3>::const_iterator>(a1), a2);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator
f17(std::flat_set<T1, T2, T3> &a0,
    typename std::flat_set<T1, T2, T3>::const_iterator a1, T1 &&a2) {
  return a0.insert(
      std::forward<typename std::flat_set<T1, T2, T3>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f18(std::flat_set<T1, T2, T3> &a0, std::initializer_list<T1> a1) {
  return a0.insert(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2, typename T3>
void f19(std::flat_set<T1, T2, T3> &a0, std::sorted_unique_t a1,
         std::initializer_list<T1> a2) {
  return a0.insert(std::move(a1), std::forward<std::initializer_list<T1>>(a2));
}

template <typename T1, typename T2, typename T3>
T3 f20(std::flat_set<T1, T2, T3> &&a0) {
  return std::move(a0).extract();
}

template <typename T1, typename T2, typename T3>
void f21(std::flat_set<T1, T2, T3> &a0, T3 &&a1) {
  return a0.replace(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator
f22(std::flat_set<T1, T2, T3> &a0,
    typename std::flat_set<T1, T2, T3>::const_iterator a1) {
  return a0.erase(
      std::forward<typename std::flat_set<T1, T2, T3>::const_iterator>(a1));
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::size_type f23(std::flat_set<T1, T2, T3> &a0,
                                                  T1 const &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator
f24(std::flat_set<T1, T2, T3> &a0,
    typename std::flat_set<T1, T2, T3>::const_iterator a1,
    typename std::flat_set<T1, T2, T3>::const_iterator a2) {
  return a0.erase(
      std::forward<typename std::flat_set<T1, T2, T3>::const_iterator>(a1),
      std::forward<typename std::flat_set<T1, T2, T3>::const_iterator>(a2));
}

template <typename T1, typename T2, typename T3>
void f25(std::flat_set<T1, T2, T3> &a0, std::flat_set<T1, T2, T3> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
void f26(std::flat_set<T1, T2, T3> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3>
T2 f27(const std::flat_set<T1, T2, T3> &a0) {
  return a0.key_comp();
}

template <typename T1, typename T2, typename T3>
T2 f28(const std::flat_set<T1, T2, T3> &a0) {
  return a0.value_comp();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator f29(std::flat_set<T1, T2, T3> &a0,
                                                 T1 const &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f30(const std::flat_set<T1, T2, T3> &a0, T1 const &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::size_type
f31(const std::flat_set<T1, T2, T3> &a0, T1 const &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3>
bool f32(const std::flat_set<T1, T2, T3> &a0, T1 const &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator f33(std::flat_set<T1, T2, T3> &a0,
                                                 T1 const &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f34(const std::flat_set<T1, T2, T3> &a0, T1 const &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator f35(std::flat_set<T1, T2, T3> &a0,
                                                 T1 const &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f36(const std::flat_set<T1, T2, T3> &a0, T1 const &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::flat_set<T1, T2, T3>::iterator,
          typename std::flat_set<T1, T2, T3>::iterator>
f37(std::flat_set<T1, T2, T3> &a0, T1 const &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::flat_set<T1, T2, T3>::const_iterator,
          typename std::flat_set<T1, T2, T3>::const_iterator>
f38(const std::flat_set<T1, T2, T3> &a0, T1 const &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename... T4>
std::pair<typename T3::const_iterator, bool> f39(std::flat_set<T1, T2, T3> &a0,
                                                 T4 &&...a1) {
  return a0.template emplace<T4...>(std::forward<T4>(a1)...);
}

template <typename T1, typename T2, typename T3, typename... T4>
typename std::flat_set<T1, T2, T3>::iterator
f40(std::flat_set<T1, T2, T3> &a0,
    typename std::flat_set<T1, T2, T3>::const_iterator a1, T4 &&...a2) {
  return a0.template emplace_hint<T4...>(
      std::forward<typename std::flat_set<T1, T2, T3>::const_iterator>(a1),
      std::forward<T4>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename T3::const_iterator, bool> f41(std::flat_set<T1, T2, T3> &a0,
                                                 T4 &&a1) {
  return a0.template insert<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::iterator
f42(std::flat_set<T1, T2, T3> &a0,
    typename std::flat_set<T1, T2, T3>::const_iterator a1, T4 &&a2) {
  return a0.template insert<T4>(
      std::forward<typename std::flat_set<T1, T2, T3>::const_iterator>(a1),
      std::forward<T4>(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
void f43(std::flat_set<T1, T2, T3> &a0, T4 a1, T4 a2) {
  return a0.template insert<T4>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
void f44(std::flat_set<T1, T2, T3> &a0, std::sorted_unique_t a1, T4 a2, T4 a3) {
  return a0.template insert<T4>(std::move(a1), std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
void f45(std::flat_set<T1, T2, T3> &a0, T4 &&a1) {
  return a0.template insert_range<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::size_type f46(std::flat_set<T1, T2, T3> &a0,
                                                  T4 &&a1) {
  return a0.template erase<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::iterator f47(std::flat_set<T1, T2, T3> &a0,
                                                 const T4 &a1) {
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::const_iterator
f48(const std::flat_set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::size_type
f49(const std::flat_set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template count<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f50(const std::flat_set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template contains<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::iterator f51(std::flat_set<T1, T2, T3> &a0,
                                                 const T4 &a1) {
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::const_iterator
f52(const std::flat_set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::iterator f53(std::flat_set<T1, T2, T3> &a0,
                                                 const T4 &a1) {
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::const_iterator
f54(const std::flat_set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::flat_set<T1, T2, T3>::iterator,
          typename std::flat_set<T1, T2, T3>::iterator>
f55(std::flat_set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template equal_range<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::flat_set<T1, T2, T3>::const_iterator,
          typename std::flat_set<T1, T2, T3>::const_iterator>
f56(const std::flat_set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template equal_range<T4>(a1);
}

template <typename T1, typename T2, typename T3>
void f57(std::flat_set<T1, T2, T3> &a0, std::flat_set<T1, T2, T3> &a1) {
  return swap(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f58(const std::flat_set<T1, T2, T3> &a0,
         const std::flat_set<T1, T2, T3> &a1) {
  return operator==(a0, a1);
}
