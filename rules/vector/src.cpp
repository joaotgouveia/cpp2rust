#include <algorithm>
#include <initializer_list>
#include <utility>
#include <vector>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t1 = std::vector<bool, T1>;

template <typename T1> using t2 = typename std::vector<bool, T1>::value_type;

template <typename T1> using t3 = typename std::vector<bool, T1>::size_type;

template <typename T1>
using t4 = typename std::vector<bool, T1>::difference_type;

template <typename T1>
using t5 = typename std::vector<bool, T1>::allocator_type;

template <typename T1>
typename std::vector<bool, T1>::allocator_type
f1(const std::vector<bool, T1> &a0) {
  return a0.get_allocator();
}

template <typename T1> std::vector<bool, T1> f2() {
  return std::vector<bool, T1>();
}

template <typename T1>
std::vector<bool, T1>
f3(const typename std::vector<bool, T1>::allocator_type &a0) {
  return std::vector<bool, T1>(a0);
}

template <typename T1>
std::vector<bool, T1>
f4(typename std::vector<bool, T1>::size_type a0,
   const typename std::vector<bool, T1>::allocator_type &a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <typename T1>
std::vector<bool, T1> f5(const std::vector<bool, T1> &a0) {
  return std::vector<bool, T1>(a0);
}

template <typename T1> std::vector<bool, T1> f6(std::vector<bool, T1> &&a0) {
  return std::vector<bool, T1>(std::move(a0));
}

template <typename T1>
std::vector<bool, T1>
f7(std::vector<bool, T1> &&a0,
   const typename std::vector<bool, T1>::allocator_type &a1) {
  return std::vector<bool, T1>(std::move(a0), a1);
}

template <typename T1>
std::vector<bool, T1>
f8(const std::vector<bool, T1> &a0,
   const typename std::vector<bool, T1>::allocator_type &a1) {
  return std::vector<bool, T1>(a0, a1);
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
typename std::vector<bool, T1>::iterator f11(std::vector<bool, T1> &a0) {
  return a0.begin();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f12(const std::vector<bool, T1> &a0) {
  return a0.begin();
}

template <typename T1>
typename std::vector<bool, T1>::iterator f13(std::vector<bool, T1> &a0) {
  return a0.end();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f14(const std::vector<bool, T1> &a0) {
  return a0.end();
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator
f15(std::vector<bool, T1> &a0) {
  return a0.rbegin();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f16(const std::vector<bool, T1> &a0) {
  return a0.rbegin();
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator
f17(std::vector<bool, T1> &a0) {
  return a0.rend();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f18(const std::vector<bool, T1> &a0) {
  return a0.rend();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f19(const std::vector<bool, T1> &a0) {
  return a0.cbegin();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f20(const std::vector<bool, T1> &a0) {
  return a0.cend();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f21(const std::vector<bool, T1> &a0) {
  return a0.crbegin();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f22(const std::vector<bool, T1> &a0) {
  return a0.crend();
}

template <typename T1>
typename std::vector<bool, T1>::size_type f23(const std::vector<bool, T1> &a0) {
  return a0.size();
}

template <typename T1>
typename std::vector<bool, T1>::size_type f24(const std::vector<bool, T1> &a0) {
  return a0.max_size();
}

template <typename T1>
typename std::vector<bool, T1>::size_type f25(const std::vector<bool, T1> &a0) {
  return a0.capacity();
}

template <typename T1> bool f26(const std::vector<bool, T1> &a0) {
  return a0.empty();
}

template <typename T1>
typename std::vector<bool, T1>::reference
f27(std::vector<bool, T1> &a0, typename std::vector<bool, T1>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_reference
f28(const std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1>
typename std::vector<bool, T1>::reference
f29(std::vector<bool, T1> &a0, typename std::vector<bool, T1>::size_type a1) {
  return a0.at(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_reference
f30(const std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::size_type a1) {
  return a0.at(a1);
}

template <typename T1>
void f31(std::vector<bool, T1> &a0,
         typename std::vector<bool, T1>::size_type a1) {
  return a0.reserve(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reference f32(std::vector<bool, T1> &a0) {
  return a0.front();
}

template <typename T1>
typename std::vector<bool, T1>::const_reference
f33(const std::vector<bool, T1> &a0) {
  return a0.front();
}

template <typename T1>
typename std::vector<bool, T1>::reference f34(std::vector<bool, T1> &a0) {
  return a0.back();
}

template <typename T1>
typename std::vector<bool, T1>::const_reference
f35(const std::vector<bool, T1> &a0) {
  return a0.back();
}

template <typename T1>
void f36(std::vector<bool, T1> &a0, std::vector<bool, T1> &a1) {
  return a0.swap(a1);
}

template <typename T1> void f37(std::vector<bool, T1> &a0) {
  return a0.pop_back();
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f38(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f39(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1,
    typename std::vector<bool, T1>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1> void f40(std::vector<bool, T1> &a0) {
  return a0.shrink_to_fit();
}

template <typename T1> void f41(std::vector<bool, T1> &a0) { return a0.flip(); }

template <typename T1> void f42(std::vector<bool, T1> &a0) {
  return a0.clear();
}

template <typename T1>
using t6 = typename std::vector<bool, T1>::const_reverse_iterator::pointer;

template <typename T1>
using t7 =
    typename std::vector<bool, T1>::const_reverse_iterator::difference_type;

template <typename T1>
using t8 = typename std::vector<bool, T1>::const_reverse_iterator::reference;

template <typename T1>
using t9 = typename std::vector<bool, T1>::reverse_iterator::pointer;

template <typename T1>
using t10 = typename std::vector<bool, T1>::reverse_iterator::difference_type;

template <typename T1>
using t11 = typename std::vector<bool, T1>::reverse_iterator::reference;

template <typename T1, typename T2> using t12 = std::vector<T1, T2>;

template <typename T1, typename T2>
using t13 = typename std::vector<T1, T2>::value_type;

template <typename T1, typename T2>
using t14 = typename std::vector<T1, T2>::pointer;

template <typename T1, typename T2>
using t15 = typename std::vector<T1, T2>::const_pointer;

template <typename T1, typename T2>
using t16 = typename std::vector<T1, T2>::reference;

template <typename T1, typename T2>
using t17 = typename std::vector<T1, T2>::const_reference;

template <typename T1, typename T2>
using t18 = typename std::vector<T1, T2>::size_type;

template <typename T1, typename T2>
using t19 = typename std::vector<T1, T2>::difference_type;

template <typename T1, typename T2>
using t20 = typename std::vector<T1, T2>::allocator_type;

template <typename T1, typename T2> std::vector<T1, T2> f43() {
  return std::vector<T1, T2>();
}

template <typename T1, typename T2>
std::vector<T1, T2>
f44(const typename std::vector<T1, T2>::allocator_type &a0) {
  return std::vector<T1, T2>(a0);
}

template <typename T1, typename T2>
std::vector<T1, T2>
f45(typename std::vector<T1, T2>::size_type a0,
    const typename std::vector<T1, T2>::allocator_type &a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> f46(const std::vector<T1, T2> &a0) {
  return std::vector<T1, T2>(a0);
}

template <typename T1, typename T2>
std::vector<T1, T2> f47(std::vector<T1, T2> &&a0) {
  return std::vector<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::vector<T1, T2>
f48(const std::vector<T1, T2> &a0,
    const typename std::vector<T1, T2>::allocator_type &a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::vector<T1, T2>
f49(std::vector<T1, T2> &&a0,
    const typename std::vector<T1, T2>::allocator_type &a1) {
  return std::vector<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::vector<T1, T2>
f50(std::initializer_list<typename std::vector<T1, T2>::value_type> a0,
    const typename std::vector<T1, T2>::allocator_type &a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> &f51(std::vector<T1, T2> &a0,
                         const std::vector<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> &f52(std::vector<T1, T2> &a0, std::vector<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::vector<T1, T2> &
f53(std::vector<T1, T2> &a0,
    std::initializer_list<typename std::vector<T1, T2>::value_type> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
void f54(std::vector<T1, T2> &a0,
         std::initializer_list<typename std::vector<T1, T2>::value_type> a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator f55(std::vector<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_iterator
f56(const std::vector<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator f57(std::vector<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_iterator
f58(const std::vector<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reverse_iterator f59(std::vector<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reverse_iterator
f60(const std::vector<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reverse_iterator f61(std::vector<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reverse_iterator
f62(const std::vector<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_iterator
f63(const std::vector<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_iterator
f64(const std::vector<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reverse_iterator
f65(const std::vector<T1, T2> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reverse_iterator
f66(const std::vector<T1, T2> &a0) {
  return a0.crend();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::size_type f67(const std::vector<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::size_type f68(const std::vector<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
void f69(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2> void f70(std::vector<T1, T2> &a0) {
  return a0.shrink_to_fit();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::size_type f71(const std::vector<T1, T2> &a0) {
  return a0.capacity();
}

template <typename T1, typename T2> bool f72(const std::vector<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
void f73(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.reserve(a1);
}

template <typename T1, typename T2>
void f74(std::vector<T1, T2> &a0,
         typename std::vector<T1, T2>::value_type &&a1) {
  return a0.push_back(std::move(a1));
}

template <typename T1, typename T2> void f75(std::vector<T1, T2> &a0) {
  return a0.pop_back();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f76(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
    typename std::vector<T1, T2>::value_type &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f77(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
    std::initializer_list<typename std::vector<T1, T2>::value_type> a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f78(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f79(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
    typename std::vector<T1, T2>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2>
void f80(std::vector<T1, T2> &a0, std::vector<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2> void f81(std::vector<T1, T2> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename... T3>
void f82(std::vector<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace_back<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename... T3>
typename std::vector<T1, T2>::iterator
f83(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
    T3 &&...a2) {
  return a0.template emplace<T3...>(a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2>
using t21 = typename std::vector<T1, T2>::iterator::iterator_category;

template <typename T1, typename T2>
using t22 = typename std::vector<T1, T2>::iterator::value_type;

template <typename T1, typename T2>
using t23 = typename std::vector<T1, T2>::iterator::difference_type;

template <typename T1, typename T2>
using t24 = typename std::vector<T1, T2>::iterator::reference;

template <typename T1, typename T2>
using t25 = typename std::vector<T1, T2>::iterator::pointer;

template <typename T1, typename T2>
using t26 = typename std::vector<T1, T2>::const_iterator::iterator_category;

template <typename T1, typename T2>
using t27 = typename std::vector<T1, T2>::const_iterator::value_type;

template <typename T1, typename T2>
using t28 = typename std::vector<T1, T2>::const_iterator::difference_type;

template <typename T1, typename T2>
using t29 = typename std::vector<T1, T2>::const_iterator::reference;

template <typename T1, typename T2>
using t30 = typename std::vector<T1, T2>::const_iterator::pointer;

template <typename T1, typename T2>
using t31 = typename std::vector<T1, T2>::const_reverse_iterator::pointer;

template <typename T1, typename T2>
using t32 =
    typename std::vector<T1, T2>::const_reverse_iterator::difference_type;

template <typename T1, typename T2>
using t33 = typename std::vector<T1, T2>::const_reverse_iterator::reference;

template <typename T1, typename T2>
using t34 = typename std::vector<T1, T2>::reverse_iterator::pointer;

template <typename T1, typename T2>
using t35 = typename std::vector<T1, T2>::reverse_iterator::difference_type;

template <typename T1, typename T2>
using t36 = typename std::vector<T1, T2>::reverse_iterator::reference;
