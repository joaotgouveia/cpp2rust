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
std::flat_set<T1, T2, T3> f3() {
  return std::flat_set<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::flat_set<T1, T2, T3> f4(T2 const &a0) {
  return std::flat_set<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::flat_set<T1, T2, T3> f5(T3 a0, T2 const &a1) {
  return std::flat_set<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::flat_set<T1, T2, T3> f6(std::sorted_unique_t a0, T3 a1, T2 const &a2) {
  return std::flat_set<T1, T2, T3>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
std::flat_set<T1, T2, T3> f7(std::initializer_list<T1> a0, T2 const &a1) {
  return std::flat_set<T1, T2, T3>(std::forward<std::initializer_list<T1>>(a0),
                                   a1);
}

template <typename T1, typename T2, typename T3>
std::flat_set<T1, T2, T3> f8(std::sorted_unique_t a0,
                             std::initializer_list<T1> a1, T2 const &a2) {
  return std::flat_set<T1, T2, T3>(
      std::move(a0), std::forward<std::initializer_list<T1>>(a1), a2);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f9(const std::flat_set<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f10(const std::flat_set<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_reverse_iterator
f11(const std::flat_set<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_reverse_iterator
f12(const std::flat_set<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f13(const std::flat_set<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f14(const std::flat_set<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_reverse_iterator
f15(const std::flat_set<T1, T2, T3> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_reverse_iterator
f16(const std::flat_set<T1, T2, T3> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3>
bool f17(const std::flat_set<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::size_type
f18(const std::flat_set<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::size_type
f19(const std::flat_set<T1, T2, T3> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3>
std::pair<typename T3::const_iterator, bool> f20(std::flat_set<T1, T2, T3> &a0,
                                                 T1 const &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename T3::const_iterator, bool> f21(std::flat_set<T1, T2, T3> &a0,
                                                 T1 &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator
f22(std::flat_set<T1, T2, T3> &a0,
    typename std::flat_set<T1, T2, T3>::const_iterator a1, T1 const &a2) {
  return a0.insert(
      std::forward<typename std::flat_set<T1, T2, T3>::const_iterator>(a1), a2);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator
f23(std::flat_set<T1, T2, T3> &a0,
    typename std::flat_set<T1, T2, T3>::const_iterator a1, T1 &&a2) {
  return a0.insert(
      std::forward<typename std::flat_set<T1, T2, T3>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f24(std::flat_set<T1, T2, T3> &a0, std::initializer_list<T1> a1) {
  return a0.insert(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2, typename T3>
void f25(std::flat_set<T1, T2, T3> &a0, std::sorted_unique_t a1,
         std::initializer_list<T1> a2) {
  return a0.insert(std::move(a1), std::forward<std::initializer_list<T1>>(a2));
}

template <typename T1, typename T2, typename T3>
T3 f26(std::flat_set<T1, T2, T3> &&a0) {
  return std::move(a0).extract();
}

template <typename T1, typename T2, typename T3>
void f27(std::flat_set<T1, T2, T3> &a0, T3 &&a1) {
  return a0.replace(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator
f28(std::flat_set<T1, T2, T3> &a0,
    typename std::flat_set<T1, T2, T3>::const_iterator a1) {
  return a0.erase(
      std::forward<typename std::flat_set<T1, T2, T3>::const_iterator>(a1));
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::size_type f29(std::flat_set<T1, T2, T3> &a0,
                                                  T1 const &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator
f30(std::flat_set<T1, T2, T3> &a0,
    typename std::flat_set<T1, T2, T3>::const_iterator a1,
    typename std::flat_set<T1, T2, T3>::const_iterator a2) {
  return a0.erase(
      std::forward<typename std::flat_set<T1, T2, T3>::const_iterator>(a1),
      std::forward<typename std::flat_set<T1, T2, T3>::const_iterator>(a2));
}

template <typename T1, typename T2, typename T3>
void f31(std::flat_set<T1, T2, T3> &a0, std::flat_set<T1, T2, T3> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
void f32(std::flat_set<T1, T2, T3> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3>
T2 f33(const std::flat_set<T1, T2, T3> &a0) {
  return a0.key_comp();
}

template <typename T1, typename T2, typename T3>
T2 f34(const std::flat_set<T1, T2, T3> &a0) {
  return a0.value_comp();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator f35(std::flat_set<T1, T2, T3> &a0,
                                                 T1 const &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f36(const std::flat_set<T1, T2, T3> &a0, T1 const &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::size_type
f37(const std::flat_set<T1, T2, T3> &a0, T1 const &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3>
bool f38(const std::flat_set<T1, T2, T3> &a0, T1 const &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator f39(std::flat_set<T1, T2, T3> &a0,
                                                 T1 const &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f40(const std::flat_set<T1, T2, T3> &a0, T1 const &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator f41(std::flat_set<T1, T2, T3> &a0,
                                                 T1 const &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f42(const std::flat_set<T1, T2, T3> &a0, T1 const &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::flat_set<T1, T2, T3>::iterator,
          typename std::flat_set<T1, T2, T3>::iterator>
f43(std::flat_set<T1, T2, T3> &a0, T1 const &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::flat_set<T1, T2, T3>::const_iterator,
          typename std::flat_set<T1, T2, T3>::const_iterator>
f44(const std::flat_set<T1, T2, T3> &a0, T1 const &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f45(T4 a0, T4 a1, T2 const &a2) {
  return std::flat_set<T1, T2, T3>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f46(std::sorted_unique_t a0, T4 a1, T4 a2,
                              T2 const &a3) {
  return std::flat_set<T1, T2, T3>(std::move(a0), std::move(a1), std::move(a2),
                                   a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f47(std::from_range_t a0, T4 &&a1) {
  return std::flat_set<T1, T2, T3>(std::move(a0), std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f48(std::from_range_t a0, T4 &&a1, T2 const &a2) {
  return std::flat_set<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f49(const T4 &a0) {
  return std::flat_set<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f50(T2 const &a0, const T4 &a1) {
  return std::flat_set<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f51(T3 const &a0, const T4 &a1) {
  return std::flat_set<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f52(T3 const &a0, T2 const &a1, const T4 &a2) {
  return std::flat_set<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f53(std::sorted_unique_t a0, T3 const &a1,
                              const T4 &a2) {
  return std::flat_set<T1, T2, T3>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f54(std::sorted_unique_t a0, T3 const &a1,
                              T2 const &a2, const T4 &a3) {
  return std::flat_set<T1, T2, T3>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f55(const std::flat_set<T1, T2, T3> &a0,
                              const T4 &a1) {
  return std::flat_set<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f56(std::flat_set<T1, T2, T3> &&a0, const T4 &a1) {
  return std::flat_set<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_set<T1, T2, T3> f57(T4 a0, T4 a1, const T5 &a2) {
  return std::flat_set<T1, T2, T3>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_set<T1, T2, T3> f58(T4 a0, T4 a1, T2 const &a2, const T5 &a3) {
  return std::flat_set<T1, T2, T3>(std::move(a0), std::move(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_set<T1, T2, T3> f59(std::sorted_unique_t a0, T4 a1, T4 a2,
                              const T5 &a3) {
  return std::flat_set<T1, T2, T3>(std::move(a0), std::move(a1), std::move(a2),
                                   a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_set<T1, T2, T3> f60(std::sorted_unique_t a0, T4 a1, T4 a2,
                              T2 const &a3, const T5 &a4) {
  return std::flat_set<T1, T2, T3>(std::move(a0), std::move(a1), std::move(a2),
                                   a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_set<T1, T2, T3> f61(std::from_range_t a0, T4 &&a1, const T5 &a2) {
  return std::flat_set<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_set<T1, T2, T3> f62(std::from_range_t a0, T4 &&a1, T2 const &a2,
                              const T5 &a3) {
  return std::flat_set<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f63(std::initializer_list<T1> a0, const T4 &a1) {
  return std::flat_set<T1, T2, T3>(std::forward<std::initializer_list<T1>>(a0),
                                   a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f64(std::initializer_list<T1> a0, T2 const &a1,
                              const T4 &a2) {
  return std::flat_set<T1, T2, T3>(std::forward<std::initializer_list<T1>>(a0),
                                   a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f65(std::sorted_unique_t a0,
                              std::initializer_list<T1> a1, const T4 &a2) {
  return std::flat_set<T1, T2, T3>(
      std::move(a0), std::forward<std::initializer_list<T1>>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f66(std::sorted_unique_t a0,
                              std::initializer_list<T1> a1, T2 const &a2,
                              const T4 &a3) {
  return std::flat_set<T1, T2, T3>(
      std::move(a0), std::forward<std::initializer_list<T1>>(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename... T4>
std::pair<typename T3::const_iterator, bool> f67(std::flat_set<T1, T2, T3> &a0,
                                                 T4 &&...a1) {
  return a0.template emplace<T4...>(std::forward<T4>(a1)...);
}

template <typename T1, typename T2, typename T3, typename... T4>
typename std::flat_set<T1, T2, T3>::iterator
f68(std::flat_set<T1, T2, T3> &a0,
    typename std::flat_set<T1, T2, T3>::const_iterator a1, T4 &&...a2) {
  return a0.template emplace_hint<T4...>(
      std::forward<typename std::flat_set<T1, T2, T3>::const_iterator>(a1),
      std::forward<T4>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename T3::const_iterator, bool> f69(std::flat_set<T1, T2, T3> &a0,
                                                 T4 &&a1) {
  return a0.template insert<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::iterator
f70(std::flat_set<T1, T2, T3> &a0,
    typename std::flat_set<T1, T2, T3>::const_iterator a1, T4 &&a2) {
  return a0.template insert<T4>(
      std::forward<typename std::flat_set<T1, T2, T3>::const_iterator>(a1),
      std::forward<T4>(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
void f71(std::flat_set<T1, T2, T3> &a0, T4 a1, T4 a2) {
  return a0.template insert<T4>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
void f72(std::flat_set<T1, T2, T3> &a0, std::sorted_unique_t a1, T4 a2, T4 a3) {
  return a0.template insert<T4>(std::move(a1), std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
void f73(std::flat_set<T1, T2, T3> &a0, T4 &&a1) {
  return a0.template insert_range<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::size_type f74(std::flat_set<T1, T2, T3> &a0,
                                                  T4 &&a1) {
  return a0.template erase<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::iterator f75(std::flat_set<T1, T2, T3> &a0,
                                                 const T4 &a1) {
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::const_iterator
f76(const std::flat_set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::size_type
f77(const std::flat_set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template count<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f78(const std::flat_set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template contains<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::iterator f79(std::flat_set<T1, T2, T3> &a0,
                                                 const T4 &a1) {
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::const_iterator
f80(const std::flat_set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::iterator f81(std::flat_set<T1, T2, T3> &a0,
                                                 const T4 &a1) {
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::const_iterator
f82(const std::flat_set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::flat_set<T1, T2, T3>::iterator,
          typename std::flat_set<T1, T2, T3>::iterator>
f83(std::flat_set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template equal_range<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::flat_set<T1, T2, T3>::const_iterator,
          typename std::flat_set<T1, T2, T3>::const_iterator>
f84(const std::flat_set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template equal_range<T4>(a1);
}

template <typename T1, typename T2, typename T3>
void f85(std::flat_set<T1, T2, T3> &a0, std::flat_set<T1, T2, T3> &a1) {
  return swap(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f86(const std::flat_set<T1, T2, T3> &a0,
         const std::flat_set<T1, T2, T3> &a1) {
  return operator==(a0, a1);
}
