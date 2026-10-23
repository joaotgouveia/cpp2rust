#include <algorithm>
#include <initializer_list>
#include <utility>
#include <vector>

template <typename T1> using t1 = std::vector<bool, T1>;

template <typename T1> using t2 = typename std::vector<bool, T1>::value_type;

template <typename T1> using t3 = typename std::vector<bool, T1>::size_type;

template <typename T1>
using t4 = typename std::vector<bool, T1>::difference_type;

template <typename T1>
using t5 = typename std::vector<bool, T1>::allocator_type;

template <typename T1> T1 f1(const std::vector<bool, T1> &a0) {
  return a0.get_allocator();
}

template <typename T1> std::vector<bool, T1> f2() {
  return std::vector<bool, T1>();
}

template <typename T1> std::vector<bool, T1> f3(const T1 &a0) {
  return std::vector<bool, T1>(a0);
}

template <typename T1>
std::vector<bool, T1> f4(const std::vector<bool, T1> &a0) {
  return std::vector<bool, T1>(a0);
}

template <typename T1> std::vector<bool, T1> f5(std::vector<bool, T1> &&a0) {
  return std::vector<bool, T1>(std::move(a0));
}

template <typename T1>
std::vector<bool, T1> f6(std::vector<bool, T1> &&a0, const T1 &a1) {
  return std::vector<bool, T1>(std::move(a0), a1);
}

template <typename T1>
std::vector<bool, T1> f7(const std::vector<bool, T1> &a0, const T1 &a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <typename T1>
std::vector<bool, T1> f8(std::initializer_list<bool> a0, const T1 &a1) {
  return std::vector<bool, T1>(std::move(a0), a1);
}

template <typename T1>
std::vector<bool, T1> &f9(std::vector<bool, T1> &a0,
                          const std::vector<bool, T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::vector<bool, T1> &f10(std::vector<bool, T1> &a0,
                           std::vector<bool, T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::vector<bool, T1> &f11(std::vector<bool, T1> &a0,
                           std::initializer_list<bool> a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
void f12(std::vector<bool, T1> &a0, std::initializer_list<bool> a1) {
  return a0.assign(std::move(a1));
}

template <typename T1>
typename std::vector<bool, T1>::iterator f13(std::vector<bool, T1> &a0) {
  return a0.begin();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f14(const std::vector<bool, T1> &a0) {
  return a0.begin();
}

template <typename T1>
typename std::vector<bool, T1>::iterator f15(std::vector<bool, T1> &a0) {
  return a0.end();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f16(const std::vector<bool, T1> &a0) {
  return a0.end();
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator
f17(std::vector<bool, T1> &a0) {
  return a0.rbegin();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f18(const std::vector<bool, T1> &a0) {
  return a0.rbegin();
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator
f19(std::vector<bool, T1> &a0) {
  return a0.rend();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f20(const std::vector<bool, T1> &a0) {
  return a0.rend();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f21(const std::vector<bool, T1> &a0) {
  return a0.cbegin();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f22(const std::vector<bool, T1> &a0) {
  return a0.cend();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f23(const std::vector<bool, T1> &a0) {
  return a0.crbegin();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f24(const std::vector<bool, T1> &a0) {
  return a0.crend();
}

template <typename T1> bool f25(const std::vector<bool, T1> &a0) {
  return a0.empty();
}

template <typename T1>
typename std::vector<bool, T1>::reference f26(std::vector<bool, T1> &a0) {
  return a0.front();
}

template <typename T1>
typename std::vector<bool, T1>::reference f27(std::vector<bool, T1> &a0) {
  return a0.back();
}

template <typename T1>
void f28(std::vector<bool, T1> &a0, std::vector<bool, T1> &a1) {
  return a0.swap(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f29(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1, const bool &a2) {
  return a0.insert(a1, a2);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f30(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1,
    std::initializer_list<bool> a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1> void f31(std::vector<bool, T1> &a0) {
  return a0.pop_back();
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f32(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f33(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1,
    typename std::vector<bool, T1>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1> void f34(std::vector<bool, T1> &a0) {
  return a0.shrink_to_fit();
}

template <typename T1> void f35(std::vector<bool, T1> &a0) { return a0.flip(); }

template <typename T1> void f36(std::vector<bool, T1> &a0) {
  return a0.clear();
}

template <typename T1, typename T2> using t6 = std::vector<T1, T2>;

template <typename T1, typename T2>
using t7 = typename std::vector<T1, T2>::value_type;

template <typename T1, typename T2>
using t8 = typename std::vector<T1, T2>::pointer;

template <typename T1, typename T2>
using t9 = typename std::vector<T1, T2>::const_pointer;

template <typename T1, typename T2>
using t10 = typename std::vector<T1, T2>::reference;

template <typename T1, typename T2>
using t11 = typename std::vector<T1, T2>::const_reference;

template <typename T1, typename T2>
using t12 = typename std::vector<T1, T2>::size_type;

template <typename T1, typename T2>
using t13 = typename std::vector<T1, T2>::difference_type;

template <typename T1, typename T2>
using t14 = typename std::vector<T1, T2>::allocator_type;

template <typename T1, typename T2> std::vector<T1, T2> f37() {
  return std::vector<T1, T2>();
}

template <typename T1, typename T2> std::vector<T1, T2> f38(const T2 &a0) {
  return std::vector<T1, T2>(a0);
}

template <typename T1, typename T2>
std::vector<T1, T2> f39(const std::vector<T1, T2> &a0) {
  return std::vector<T1, T2>(a0);
}

template <typename T1, typename T2>
std::vector<T1, T2> f40(std::vector<T1, T2> &&a0) {
  return std::vector<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::vector<T1, T2> f41(const std::vector<T1, T2> &a0, const T2 &a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> f42(std::vector<T1, T2> &&a0, const T2 &a1) {
  return std::vector<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> f43(std::initializer_list<T1> a0, const T2 &a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> &f44(std::vector<T1, T2> &a0,
                         const std::vector<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> &f45(std::vector<T1, T2> &a0, std::vector<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::vector<T1, T2> &f46(std::vector<T1, T2> &a0,
                         std::initializer_list<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
void f47(std::vector<T1, T2> &a0, std::initializer_list<T1> a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator f48(std::vector<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_iterator
f49(const std::vector<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator f50(std::vector<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_iterator
f51(const std::vector<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reverse_iterator f52(std::vector<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reverse_iterator
f53(const std::vector<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reverse_iterator f54(std::vector<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reverse_iterator
f55(const std::vector<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_iterator
f56(const std::vector<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_iterator
f57(const std::vector<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reverse_iterator
f58(const std::vector<T1, T2> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reverse_iterator
f59(const std::vector<T1, T2> &a0) {
  return a0.crend();
}

template <typename T1, typename T2> void f60(std::vector<T1, T2> &a0) {
  return a0.shrink_to_fit();
}

template <typename T1, typename T2> bool f61(const std::vector<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2> T1 *f62(std::vector<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
const T1 *f63(const std::vector<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
void f64(std::vector<T1, T2> &a0, const T1 &a1) {
  return a0.push_back(a1);
}

template <typename T1, typename T2> void f65(std::vector<T1, T2> &a0, T1 &&a1) {
  return a0.push_back(std::move(a1));
}

template <typename T1, typename T2> void f66(std::vector<T1, T2> &a0) {
  return a0.pop_back();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f67(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
    const T1 &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f68(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
    T1 &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f69(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
    std::initializer_list<T1> a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f70(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f71(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
    typename std::vector<T1, T2>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2>
void f72(std::vector<T1, T2> &a0, std::vector<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2> void f73(std::vector<T1, T2> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename... T3>
typename std::vector<T1, T2>::iterator
f74(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
    T3 &&...a2) {
  return a0.template emplace<T3...>(a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2>
using t15 = typename std::vector<T1, T2>::iterator::iterator_category;

template <typename T1, typename T2>
using t16 = typename std::vector<T1, T2>::iterator::value_type;

template <typename T1, typename T2>
using t17 = typename std::vector<T1, T2>::iterator::difference_type;

template <typename T1, typename T2>
using t18 = typename std::vector<T1, T2>::iterator::reference;

template <typename T1, typename T2>
using t19 = typename std::vector<T1, T2>::iterator::pointer;

template <typename T1, typename T2>
using t20 = typename std::vector<T1, T2>::const_iterator::iterator_category;

template <typename T1, typename T2>
using t21 = typename std::vector<T1, T2>::const_iterator::value_type;

template <typename T1, typename T2>
using t22 = typename std::vector<T1, T2>::const_iterator::difference_type;

template <typename T1, typename T2>
using t23 = typename std::vector<T1, T2>::const_iterator::reference;

template <typename T1, typename T2>
using t24 = typename std::vector<T1, T2>::const_iterator::pointer;
