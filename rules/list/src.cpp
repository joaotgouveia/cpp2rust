#include <algorithm>
#include <initializer_list>
#include <list>
#include <utility>

template <typename T1, typename T2> using t1 = std::list<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::list<T1, T2>::value_type;

template <typename T1, typename T2>
using t3 = typename std::list<T1, T2>::pointer;

template <typename T1, typename T2>
using t4 = typename std::list<T1, T2>::const_pointer;

template <typename T1, typename T2>
using t5 = typename std::list<T1, T2>::reference;

template <typename T1, typename T2>
using t6 = typename std::list<T1, T2>::const_reference;

template <typename T1, typename T2>
using t7 = typename std::list<T1, T2>::iterator;

template <typename T1, typename T2>
using t8 = typename std::list<T1, T2>::const_iterator;

template <typename T1, typename T2>
using t9 = typename std::list<T1, T2>::const_reverse_iterator;

template <typename T1, typename T2>
using t10 = typename std::list<T1, T2>::reverse_iterator;

template <typename T1, typename T2>
using t11 = typename std::list<T1, T2>::size_type;

template <typename T1, typename T2>
using t12 = typename std::list<T1, T2>::difference_type;

template <typename T1, typename T2>
using t13 = typename std::list<T1, T2>::allocator_type;

template <typename T1, typename T2> std::list<T1, T2> f1() {
  return std::list<T1, T2>();
}

template <typename T1, typename T2> std::list<T1, T2> f2(const T2 &a0) {
  return std::list<T1, T2>(a0);
}

template <typename T1, typename T2>
std::list<T1, T2> f3(const std::list<T1, T2> &a0) {
  return std::list<T1, T2>(a0);
}

template <typename T1, typename T2>
std::list<T1, T2> f4(std::list<T1, T2> &&a0) {
  return std::list<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::list<T1, T2> f5(std::initializer_list<T1> a0, const T2 &a1) {
  return std::list<T1, T2>(std::forward<std::initializer_list<T1>>(a0), a1);
}

template <typename T1, typename T2>
std::list<T1, T2> f6(const std::list<T1, T2> &a0, const T2 &a1) {
  return std::list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::list<T1, T2> f7(std::list<T1, T2> &&a0, const T2 &a1) {
  return std::list<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::list<T1, T2> &f8(std::list<T1, T2> &a0, const std::list<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::list<T1, T2> &f9(std::list<T1, T2> &a0, std::list<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::list<T1, T2> &f10(std::list<T1, T2> &a0, std::initializer_list<T1> a1) {
  return a0.operator=(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2>
void f11(std::list<T1, T2> &a0, std::initializer_list<T1> a1) {
  return a0.assign(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2> T2 f12(const std::list<T1, T2> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator f13(std::list<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_iterator f14(const std::list<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator f15(std::list<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_iterator f16(const std::list<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::reverse_iterator f17(std::list<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_reverse_iterator
f18(const std::list<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::reverse_iterator f19(std::list<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_reverse_iterator
f20(const std::list<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_iterator f21(const std::list<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_iterator f22(const std::list<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_reverse_iterator
f23(const std::list<T1, T2> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_reverse_iterator
f24(const std::list<T1, T2> &a0) {
  return a0.crend();
}

template <typename T1, typename T2> bool f25(const std::list<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
void f26(std::list<T1, T2> &a0, const T1 &a1) {
  return a0.push_front(a1);
}

template <typename T1, typename T2> void f27(std::list<T1, T2> &a0, T1 &&a1) {
  return a0.push_front(std::move(a1));
}

template <typename T1, typename T2> void f28(std::list<T1, T2> &a0) {
  return a0.pop_front();
}

template <typename T1, typename T2>
void f29(std::list<T1, T2> &a0, const T1 &a1) {
  return a0.push_back(a1);
}

template <typename T1, typename T2> void f30(std::list<T1, T2> &a0, T1 &&a1) {
  return a0.push_back(std::move(a1));
}

template <typename T1, typename T2> void f31(std::list<T1, T2> &a0) {
  return a0.pop_back();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator
f32(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
    const T1 &a2) {
  return a0.insert(std::forward<typename std::list<T1, T2>::const_iterator>(a1),
                   a2);
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator
f33(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
    T1 &&a2) {
  return a0.insert(std::forward<typename std::list<T1, T2>::const_iterator>(a1),
                   std::move(a2));
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator
f34(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
    std::initializer_list<T1> a2) {
  return a0.insert(std::forward<typename std::list<T1, T2>::const_iterator>(a1),
                   std::forward<std::initializer_list<T1>>(a2));
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator
f35(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1) {
  return a0.erase(std::forward<typename std::list<T1, T2>::const_iterator>(a1));
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator
f36(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
    typename std::list<T1, T2>::const_iterator a2) {
  return a0.erase(std::forward<typename std::list<T1, T2>::const_iterator>(a1),
                  std::forward<typename std::list<T1, T2>::const_iterator>(a2));
}

template <typename T1, typename T2>
void f37(std::list<T1, T2> &a0, std::list<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2> void f38(std::list<T1, T2> &a0) {
  return a0.clear();
}

template <typename T1, typename T2>
void f39(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
         std::list<T1, T2> &&a2) {
  return a0.splice(std::forward<typename std::list<T1, T2>::const_iterator>(a1),
                   std::move(a2));
}

template <typename T1, typename T2>
void f40(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
         std::list<T1, T2> &a2) {
  return a0.splice(std::forward<typename std::list<T1, T2>::const_iterator>(a1),
                   a2);
}

template <typename T1, typename T2>
void f41(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
         std::list<T1, T2> &&a2,
         typename std::list<T1, T2>::const_iterator a3) {
  return a0.splice(
      std::forward<typename std::list<T1, T2>::const_iterator>(a1),
      std::move(a2),
      std::forward<typename std::list<T1, T2>::const_iterator>(a3));
}

template <typename T1, typename T2>
void f42(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
         std::list<T1, T2> &a2, typename std::list<T1, T2>::const_iterator a3) {
  return a0.splice(
      std::forward<typename std::list<T1, T2>::const_iterator>(a1), a2,
      std::forward<typename std::list<T1, T2>::const_iterator>(a3));
}

template <typename T1, typename T2>
void f43(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
         std::list<T1, T2> &&a2, typename std::list<T1, T2>::const_iterator a3,
         typename std::list<T1, T2>::const_iterator a4) {
  return a0.splice(
      std::forward<typename std::list<T1, T2>::const_iterator>(a1),
      std::move(a2),
      std::forward<typename std::list<T1, T2>::const_iterator>(a3),
      std::forward<typename std::list<T1, T2>::const_iterator>(a4));
}

template <typename T1, typename T2>
void f44(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
         std::list<T1, T2> &a2, typename std::list<T1, T2>::const_iterator a3,
         typename std::list<T1, T2>::const_iterator a4) {
  return a0.splice(
      std::forward<typename std::list<T1, T2>::const_iterator>(a1), a2,
      std::forward<typename std::list<T1, T2>::const_iterator>(a3),
      std::forward<typename std::list<T1, T2>::const_iterator>(a4));
}

template <typename T1, typename T2>
void f45(std::list<T1, T2> &a0, std::list<T1, T2> &&a1) {
  return a0.merge(std::move(a1));
}

template <typename T1, typename T2>
void f46(std::list<T1, T2> &a0, std::list<T1, T2> &a1) {
  return a0.merge(a1);
}

template <typename T1, typename T2> void f47(std::list<T1, T2> &a0) {
  return a0.reverse();
}

template <typename T1, typename T2> void f48(std::list<T1, T2> &a0) {
  return a0.sort();
}

template <typename T1, typename T2, typename... T3>
typename std::list<T1, T2>::iterator
f49(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
    T3 &&...a2) {
  return a0.template emplace<T3...>(
      std::forward<typename std::list<T1, T2>::const_iterator>(a1),
      std::forward<T3>(a2)...);
}

template <typename T1, typename T2, typename T3>
void f50(std::list<T1, T2> &a0, std::list<T1, T2> &&a1, T3 a2) {
  return a0.template merge<T3>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f51(std::list<T1, T2> &a0, std::list<T1, T2> &a1, T3 a2) {
  return a0.template merge<T3>(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f52(std::list<T1, T2> &a0, T3 a1) {
  return a0.template sort<T3>(std::move(a1));
}
