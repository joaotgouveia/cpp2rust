#include <algorithm>
#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::map<T1, T2, T3, T4>::size_type f1(std::map<T1, T2, T3, T4> &a0,
                                                T5 a1) {
  return std::erase_if(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::multimap<T1, T2, T3, T4>::size_type
f2(std::multimap<T1, T2, T3, T4> &a0, T5 a1) {
  return std::erase_if(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
using t1 = std::map<T1, T2, T3, T4>;

template <typename T1, typename T2, typename T3, typename T4>
using t2 = typename std::map<T1, T2, T3, T4>::key_type;

template <typename T1, typename T2, typename T3, typename T4>
using t3 = typename std::map<T1, T2, T3, T4>::mapped_type;

template <typename T1, typename T2, typename T3, typename T4>
using t4 = typename std::map<T1, T2, T3, T4>::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t5 = typename std::map<T1, T2, T3, T4>::key_compare;

template <typename T1, typename T2, typename T3, typename T4>
using t6 = typename std::map<T1, T2, T3, T4>::allocator_type;

template <typename T1, typename T2, typename T3, typename T4>
using t7 = typename std::map<T1, T2, T3, T4>::value_compare;

template <typename T1, typename T2, typename T3, typename T4>
using t8 = typename std::map<T1, T2, T3, T4>::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t9 = typename std::map<T1, T2, T3, T4>::const_pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t10 = typename std::map<T1, T2, T3, T4>::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t11 = typename std::map<T1, T2, T3, T4>::const_reference;

template <typename T1, typename T2, typename T3, typename T4>
using t12 = typename std::map<T1, T2, T3, T4>::iterator;

template <typename T1, typename T2, typename T3, typename T4>
using t13 = typename std::map<T1, T2, T3, T4>::const_iterator;

template <typename T1, typename T2, typename T3, typename T4>
using t14 = typename std::map<T1, T2, T3, T4>::size_type;

template <typename T1, typename T2, typename T3, typename T4>
using t15 = typename std::map<T1, T2, T3, T4>::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t16 = typename std::map<T1, T2, T3, T4>::reverse_iterator;

template <typename T1, typename T2, typename T3, typename T4>
using t17 = typename std::map<T1, T2, T3, T4>::const_reverse_iterator;

template <typename T1, typename T2, typename T3, typename T4>
using t18 = typename std::map<T1, T2, T3, T4>::node_type;

template <typename T1, typename T2, typename T3, typename T4>
using t19 = typename std::map<T1, T2, T3, T4>::insert_return_type;

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f3() {
  return std::map<T1, T2, T3, T4>();
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f4(const T3 &a0, const T4 &a1) {
  return std::map<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f5(const std::map<T1, T2, T3, T4> &a0) {
  return std::map<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f6(std::map<T1, T2, T3, T4> &&a0) {
  return std::map<T1, T2, T3, T4>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f7(std::initializer_list<std::pair<const T1, T2>> a0,
                            const T3 &a1, const T4 &a2) {
  return std::map<T1, T2, T3, T4>(
      std::forward<std::initializer_list<std::pair<const T1, T2>>>(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f8(const T4 &a0) {
  return std::map<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f9(const std::map<T1, T2, T3, T4> &a0, const T4 &a1) {
  return std::map<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f10(std::map<T1, T2, T3, T4> &&a0, const T4 &a1) {
  return std::map<T1, T2, T3, T4>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f11(std::initializer_list<std::pair<const T1, T2>> a0,
                             const T4 &a1) {
  return std::map<T1, T2, T3, T4>(
      std::forward<std::initializer_list<std::pair<const T1, T2>>>(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> &f12(std::map<T1, T2, T3, T4> &a0,
                              const std::map<T1, T2, T3, T4> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> &f13(std::map<T1, T2, T3, T4> &a0,
                              std::map<T1, T2, T3, T4> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> &
f14(std::map<T1, T2, T3, T4> &a0,
    std::initializer_list<std::pair<const T1, T2>> a1) {
  return a0.operator=(
      std::forward<std::initializer_list<std::pair<const T1, T2>>>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
T4 f15(const std::map<T1, T2, T3, T4> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator f16(std::map<T1, T2, T3, T4> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_iterator
f17(const std::map<T1, T2, T3, T4> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator f18(std::map<T1, T2, T3, T4> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_iterator
f19(const std::map<T1, T2, T3, T4> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::reverse_iterator
f20(std::map<T1, T2, T3, T4> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_reverse_iterator
f21(const std::map<T1, T2, T3, T4> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::reverse_iterator
f22(std::map<T1, T2, T3, T4> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_reverse_iterator
f23(const std::map<T1, T2, T3, T4> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_iterator
f24(const std::map<T1, T2, T3, T4> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_iterator
f25(const std::map<T1, T2, T3, T4> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_reverse_iterator
f26(const std::map<T1, T2, T3, T4> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_reverse_iterator
f27(const std::map<T1, T2, T3, T4> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3, typename T4>
bool f28(const std::map<T1, T2, T3, T4> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::size_type
f29(const std::map<T1, T2, T3, T4> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::size_type
f30(const std::map<T1, T2, T3, T4> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3, typename T4>
T2 &f31(std::map<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3, typename T4>
T2 &f32(std::map<T1, T2, T3, T4> &a0, T1 &&a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
T2 &f33(std::map<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
const T2 &f34(const std::map<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::node_type
f35(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1) {
  return a0.extract(
      std::forward<typename std::map<T1, T2, T3, T4>::const_iterator>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::node_type f36(std::map<T1, T2, T3, T4> &a0,
                                                 const T1 &a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::insert_return_type
f37(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::node_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator
f38(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1,
    typename std::map<T1, T2, T3, T4>::node_type &&a2) {
  return a0.insert(
      std::forward<typename std::map<T1, T2, T3, T4>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f39(std::map<T1, T2, T3, T4> &a0, const std::pair<const T1, T2> &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f40(std::map<T1, T2, T3, T4> &a0, std::pair<const T1, T2> &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
void f41(std::map<T1, T2, T3, T4> &a0,
         std::initializer_list<std::pair<const T1, T2>> a1) {
  return a0.insert(
      std::forward<std::initializer_list<std::pair<const T1, T2>>>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator
f42(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1,
    const std::pair<const T1, T2> &a2) {
  return a0.insert(
      std::forward<typename std::map<T1, T2, T3, T4>::const_iterator>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator
f43(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1,
    std::pair<const T1, T2> &&a2) {
  return a0.insert(
      std::forward<typename std::map<T1, T2, T3, T4>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator
f44(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1) {
  return a0.erase(
      std::forward<typename std::map<T1, T2, T3, T4>::const_iterator>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator
f45(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::iterator a1) {
  return a0.erase(
      std::forward<typename std::map<T1, T2, T3, T4>::iterator>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::size_type f46(std::map<T1, T2, T3, T4> &a0,
                                                 const T1 &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator
f47(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1,
    typename std::map<T1, T2, T3, T4>::const_iterator a2) {
  return a0.erase(
      std::forward<typename std::map<T1, T2, T3, T4>::const_iterator>(a1),
      std::forward<typename std::map<T1, T2, T3, T4>::const_iterator>(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
void f48(std::map<T1, T2, T3, T4> &a0, std::map<T1, T2, T3, T4> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f49(std::map<T1, T2, T3, T4> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f50(const std::map<T1, T2, T3, T4> &a0) {
  return a0.key_comp();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::value_compare
f51(const std::map<T1, T2, T3, T4> &a0) {
  return a0.value_comp();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator f52(std::map<T1, T2, T3, T4> &a0,
                                                const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_iterator
f53(const std::map<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::size_type
f54(const std::map<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f55(const std::map<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator f56(std::map<T1, T2, T3, T4> &a0,
                                                const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_iterator
f57(const std::map<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator f58(std::map<T1, T2, T3, T4> &a0,
                                                const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_iterator
f59(const std::map<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::map<T1, T2, T3, T4>::iterator,
          typename std::map<T1, T2, T3, T4>::iterator>
f60(std::map<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::map<T1, T2, T3, T4>::const_iterator,
          typename std::map<T1, T2, T3, T4>::const_iterator>
f61(const std::map<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::map<T1, T2, T3, T4> f62(T5 a0, T5 a1, const T4 &a2) {
  return std::map<T1, T2, T3, T4>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::map<T1, T2, T3, T4> f63(T5 a0, T5 a1) {
  return std::map<T1, T2, T3, T4>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::map<T1, T2, T3, T4> f64(T5 a0, T5 a1, const T3 &a2, const T4 &a3) {
  return std::map<T1, T2, T3, T4>(std::move(a0), std::move(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::map<T1, T2, T3, T4> f65(std::from_range_t a0, T5 &&a1, const T3 &a2,
                             const T4 &a3) {
  return std::map<T1, T2, T3, T4>(std::move(a0), std::forward<T5>(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::map<T1, T2, T3, T4> f66(std::from_range_t a0, T5 &&a1, const T4 &a2) {
  return std::map<T1, T2, T3, T4>(std::move(a0), std::forward<T5>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f67(std::map<T1, T2, T3, T4> &a0, T5 &&...a1) {
  return a0.template emplace<T5...>(std::forward<T5>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
typename std::map<T1, T2, T3, T4>::iterator
f68(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1, T5 &&...a2) {
  return a0.template emplace_hint<T5...>(
      std::forward<typename std::map<T1, T2, T3, T4>::const_iterator>(a1),
      std::forward<T5>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f69(std::map<T1, T2, T3, T4> &a0, std::map<T1, T2, T5, T4> &a1) {
  return a0.template merge<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f70(std::map<T1, T2, T3, T4> &a0, std::map<T1, T2, T5, T4> &&a1) {
  return a0.template merge<T5>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f71(std::map<T1, T2, T3, T4> &a0, std::multimap<T1, T2, T5, T4> &a1) {
  return a0.template merge<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f72(std::map<T1, T2, T3, T4> &a0, std::multimap<T1, T2, T5, T4> &&a1) {
  return a0.template merge<T5>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f73(std::map<T1, T2, T3, T4> &a0, const T1 &a1, T5 &&...a2) {
  return a0.template try_emplace<T5...>(a1, std::forward<T5>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f74(std::map<T1, T2, T3, T4> &a0, T1 &&a1, T5 &&...a2) {
  return a0.template try_emplace<T5...>(std::move(a1), std::forward<T5>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
typename std::map<T1, T2, T3, T4>::iterator
f75(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1, const T1 &a2,
    T5 &&...a3) {
  return a0.template try_emplace<T5...>(
      std::forward<typename std::map<T1, T2, T3, T4>::const_iterator>(a1), a2,
      std::forward<T5>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
typename std::map<T1, T2, T3, T4>::iterator
f76(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1, T1 &&a2, T5 &&...a3) {
  return a0.template try_emplace<T5...>(
      std::forward<typename std::map<T1, T2, T3, T4>::const_iterator>(a1),
      std::move(a2), std::forward<T5>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__enable_if_t<
    std::is_constructible<std::pair<const T1, T2>, T5>::value,
    std::pair<typename std::_Rb_tree<
                  T1, std::pair<const T1, T2>,
                  std::_Select1st<std::pair<const T1, T2>>, T3,
                  typename __gnu_cxx::__alloc_traits<T4>::template rebind<
                      std::pair<const T1, T2>>::other>::iterator,
              bool>>
f77(std::map<T1, T2, T3, T4> &a0, T5 &&a1)
  requires(
      !(std::is_same<T5,
                     typename std::map<T1, T2, T3, T4>::node_type>::value) &&
      !(std::is_same<
          T5, const typename std::map<T1, T2, T3, T4>::value_type &>::value) &&
      !(std::is_same<T5,
                     typename std::map<T1, T2, T3, T4>::value_type>::value) &&
      !(std::is_same<T5, std::initializer_list<typename std::map<
                             T1, T2, T3, T4>::value_type>>::value))
{
  return a0.template insert<T5>(std::forward<T5>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f78(std::map<T1, T2, T3, T4> &a0, T5 &&a1) {
  return a0.template insert_range<T5>(std::forward<T5>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__enable_if_t<std::is_constructible<std::pair<const T1, T2>, T5>::value,
                   typename std::_Rb_tree<
                       T1, std::pair<const T1, T2>,
                       std::_Select1st<std::pair<const T1, T2>>, T3,
                       typename __gnu_cxx::__alloc_traits<T4>::template rebind<
                           std::pair<const T1, T2>>::other>::iterator>
f79(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1, T5 &&a2)
  requires(
      !(std::is_same<T5,
                     typename std::map<T1, T2, T3, T4>::node_type>::value) &&
      !(std::is_same<
          T5, const typename std::map<T1, T2, T3, T4>::value_type &>::value) &&
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::value_type>::value))
{
  return a0.template insert<T5>(
      std::forward<typename std::map<T1, T2, T3, T4>::const_iterator>(a1),
      std::forward<T5>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f80(std::map<T1, T2, T3, T4> &a0, T5 a1, T5 a2) {
  return a0.template insert<T5>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f81(std::map<T1, T2, T3, T4> &a0, const T1 &a1, T5 &&a2) {
  return a0.template insert_or_assign<T5>(a1, std::forward<T5>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f82(std::map<T1, T2, T3, T4> &a0, T1 &&a1, T5 &&a2) {
  return a0.template insert_or_assign<T5>(std::move(a1), std::forward<T5>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::map<T1, T2, T3, T4>::iterator
f83(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1, const T1 &a2,
    T5 &&a3) {
  return a0.template insert_or_assign<T5>(
      std::forward<typename std::map<T1, T2, T3, T4>::const_iterator>(a1), a2,
      std::forward<T5>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::map<T1, T2, T3, T4>::iterator
f84(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1, T1 &&a2, T5 &&a3) {
  return a0.template insert_or_assign<T5>(
      std::forward<typename std::map<T1, T2, T3, T4>::const_iterator>(a1),
      std::move(a2), std::forward<T5>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f85(std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template find<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f86(const std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template find<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f87(const std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template count<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f88(const std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template contains<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f89(std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template lower_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f90(const std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template lower_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f91(std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template upper_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f92(const std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template upper_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f93(std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template equal_range<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f94(const std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template equal_range<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
using t20 = typename std::map<T1, T2, T3, T4>::iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t21 = typename std::map<T1, T2, T3, T4>::iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t22 = typename std::map<T1, T2, T3, T4>::iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t23 = typename std::map<T1, T2, T3, T4>::iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t24 = typename std::map<T1, T2, T3, T4>::iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t25 = typename std::map<T1, T2, T3, T4>::const_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t26 = typename std::map<T1, T2, T3, T4>::const_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t27 = typename std::map<T1, T2, T3, T4>::const_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t28 =
    typename std::map<T1, T2, T3, T4>::const_iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t29 = typename std::map<T1, T2, T3, T4>::const_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
bool f95(const typename std::map<T1, T2, T3, T4>::value_compare &a0,
         const std::pair<const T1, T2> &a1, const std::pair<const T1, T2> &a2) {
  return a0.operator()(a1, a2);
}

template <typename T1, typename T2, typename T3>
using t30 = std::pmr::map<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t31 = std::pmr::multimap<T1, T2, T3>;
