#include <algorithm>
#include <initializer_list>
#include <map>
#include <utility>

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
std::map<T1, T2, T3, T4> f1() {
  return std::map<T1, T2, T3, T4>();
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f2(const T3 &a0, const T4 &a1) {
  return std::map<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f3(const std::map<T1, T2, T3, T4> &a0) {
  return std::map<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f4(std::map<T1, T2, T3, T4> &&a0) {
  return std::map<T1, T2, T3, T4>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f5(std::initializer_list<std::pair<const T1, T2>> a0,
                            const T3 &a1, const T4 &a2) {
  return std::map<T1, T2, T3, T4>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f6(const T4 &a0) {
  return std::map<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f7(const std::map<T1, T2, T3, T4> &a0, const T4 &a1) {
  return std::map<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f8(std::map<T1, T2, T3, T4> &&a0, const T4 &a1) {
  return std::map<T1, T2, T3, T4>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f9(std::initializer_list<std::pair<const T1, T2>> a0,
                            const T4 &a1) {
  return std::map<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> &f10(std::map<T1, T2, T3, T4> &a0,
                              const std::map<T1, T2, T3, T4> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> &f11(std::map<T1, T2, T3, T4> &a0,
                              std::map<T1, T2, T3, T4> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> &
f12(std::map<T1, T2, T3, T4> &a0,
    std::initializer_list<std::pair<const T1, T2>> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
T4 f13(const std::map<T1, T2, T3, T4> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator f14(std::map<T1, T2, T3, T4> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_iterator
f15(const std::map<T1, T2, T3, T4> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator f16(std::map<T1, T2, T3, T4> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_iterator
f17(const std::map<T1, T2, T3, T4> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::reverse_iterator
f18(std::map<T1, T2, T3, T4> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_reverse_iterator
f19(const std::map<T1, T2, T3, T4> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::reverse_iterator
f20(std::map<T1, T2, T3, T4> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_reverse_iterator
f21(const std::map<T1, T2, T3, T4> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_iterator
f22(const std::map<T1, T2, T3, T4> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_iterator
f23(const std::map<T1, T2, T3, T4> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_reverse_iterator
f24(const std::map<T1, T2, T3, T4> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_reverse_iterator
f25(const std::map<T1, T2, T3, T4> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3, typename T4>
bool f26(const std::map<T1, T2, T3, T4> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::size_type
f27(const std::map<T1, T2, T3, T4> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::size_type
f28(const std::map<T1, T2, T3, T4> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3, typename T4>
T2 &f29(std::map<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3, typename T4>
T2 &f30(std::map<T1, T2, T3, T4> &a0, T1 &&a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
T2 &f31(std::map<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
const T2 &f32(const std::map<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f33(std::map<T1, T2, T3, T4> &a0, const std::pair<const T1, T2> &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f34(std::map<T1, T2, T3, T4> &a0, std::pair<const T1, T2> &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
void f35(std::map<T1, T2, T3, T4> &a0,
         std::initializer_list<std::pair<const T1, T2>> a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator
f36(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1,
    const std::pair<const T1, T2> &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator
f37(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1,
    std::pair<const T1, T2> &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator
f38(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator
f39(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::size_type f40(std::map<T1, T2, T3, T4> &a0,
                                                 const T1 &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator
f41(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1,
    typename std::map<T1, T2, T3, T4>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
void f42(std::map<T1, T2, T3, T4> &a0, std::map<T1, T2, T3, T4> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f43(std::map<T1, T2, T3, T4> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f44(const std::map<T1, T2, T3, T4> &a0) {
  return a0.key_comp();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::value_compare
f45(const std::map<T1, T2, T3, T4> &a0) {
  return a0.value_comp();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator f46(std::map<T1, T2, T3, T4> &a0,
                                                const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_iterator
f47(const std::map<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::size_type
f48(const std::map<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator f49(std::map<T1, T2, T3, T4> &a0,
                                                const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_iterator
f50(const std::map<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator f51(std::map<T1, T2, T3, T4> &a0,
                                                const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_iterator
f52(const std::map<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::map<T1, T2, T3, T4>::iterator,
          typename std::map<T1, T2, T3, T4>::iterator>
f53(std::map<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::map<T1, T2, T3, T4>::const_iterator,
          typename std::map<T1, T2, T3, T4>::const_iterator>
f54(const std::map<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::map<T1, T2, T3, T4> f55(T5 a0, T5 a1, const T4 &a2) {
  return std::map<T1, T2, T3, T4>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::map<T1, T2, T3, T4> f56(T5 a0, T5 a1, const T3 &a2, const T4 &a3) {
  return std::map<T1, T2, T3, T4>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f57(std::map<T1, T2, T3, T4> &a0, T5 &&...a1) {
  return a0.template emplace<T5...>(std::forward<T5>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
typename std::map<T1, T2, T3, T4>::iterator
f58(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1, T5 &&...a2) {
  return a0.template emplace_hint<T5...>(a1, std::forward<T5>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f59(std::map<T1, T2, T3, T4> &a0, T5 a1, T5 a2) {
  return a0.template insert<T5>(a1, a2);
}
