#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <utility>
#include <vector>

template <typename T1> using t1 = std::vector<bool, T1>;

template <typename T1> using t2 = typename std::vector<bool, T1>::value_type;

template <typename T1> using t3 = typename std::vector<bool, T1>::size_type;

template <typename T1>
using t4 = typename std::vector<bool, T1>::difference_type;

template <typename T1> using t5 = typename std::vector<bool, T1>::reference;

template <typename T1>
using t6 = typename std::vector<bool, T1>::const_reference;

template <typename T1> using t7 = typename std::vector<bool, T1>::pointer;

template <typename T1> using t8 = typename std::vector<bool, T1>::const_pointer;

template <typename T1> using t9 = typename std::vector<bool, T1>::iterator;

template <typename T1>
using t10 = typename std::vector<bool, T1>::const_iterator;

template <typename T1>
using t11 = typename std::vector<bool, T1>::const_reverse_iterator;

template <typename T1>
using t12 = typename std::vector<bool, T1>::reverse_iterator;

template <typename T1>
using t13 = typename std::vector<bool, T1>::allocator_type;

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
std::vector<bool, T1>
f5(typename std::vector<bool, T1>::size_type a0, const bool &a1,
   const typename std::vector<bool, T1>::allocator_type &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <typename T1>
std::vector<bool, T1> f6(const std::vector<bool, T1> &a0) {
  return std::vector<bool, T1>(a0);
}

template <typename T1> std::vector<bool, T1> f7(std::vector<bool, T1> &&a0) {
  return std::vector<bool, T1>(std::move(a0));
}

template <typename T1>
std::vector<bool, T1>
f8(std::vector<bool, T1> &&a0,
   const typename std::vector<bool, T1>::allocator_type &a1) {
  return std::vector<bool, T1>(std::move(a0), a1);
}

template <typename T1>
std::vector<bool, T1>
f9(const std::vector<bool, T1> &a0,
   const typename std::vector<bool, T1>::allocator_type &a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <typename T1>
std::vector<bool, T1>
f10(std::initializer_list<bool> a0,
    const typename std::vector<bool, T1>::allocator_type &a1) {
  return std::vector<bool, T1>(std::move(a0), a1);
}

template <typename T1>
std::vector<bool, T1> &f11(std::vector<bool, T1> &a0,
                           const std::vector<bool, T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::vector<bool, T1> &f12(std::vector<bool, T1> &a0,
                           std::vector<bool, T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::vector<bool, T1> &f13(std::vector<bool, T1> &a0,
                           std::initializer_list<bool> a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
void f14(std::vector<bool, T1> &a0,
         typename std::vector<bool, T1>::size_type a1, const bool &a2) {
  return a0.assign(a1, a2);
}

template <typename T1>
void f15(std::vector<bool, T1> &a0, std::initializer_list<bool> a1) {
  return a0.assign(std::move(a1));
}

template <typename T1>
typename std::vector<bool, T1>::iterator f16(std::vector<bool, T1> &a0) {
  return a0.begin();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f17(const std::vector<bool, T1> &a0) {
  return a0.begin();
}

template <typename T1>
typename std::vector<bool, T1>::iterator f18(std::vector<bool, T1> &a0) {
  return a0.end();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f19(const std::vector<bool, T1> &a0) {
  return a0.end();
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator
f20(std::vector<bool, T1> &a0) {
  return a0.rbegin();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f21(const std::vector<bool, T1> &a0) {
  return a0.rbegin();
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator
f22(std::vector<bool, T1> &a0) {
  return a0.rend();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f23(const std::vector<bool, T1> &a0) {
  return a0.rend();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f24(const std::vector<bool, T1> &a0) {
  return a0.cbegin();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f25(const std::vector<bool, T1> &a0) {
  return a0.cend();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f26(const std::vector<bool, T1> &a0) {
  return a0.crbegin();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f27(const std::vector<bool, T1> &a0) {
  return a0.crend();
}

template <typename T1>
typename std::vector<bool, T1>::size_type f28(const std::vector<bool, T1> &a0) {
  return a0.size();
}

template <typename T1>
typename std::vector<bool, T1>::size_type f29(const std::vector<bool, T1> &a0) {
  return a0.max_size();
}

template <typename T1>
typename std::vector<bool, T1>::size_type f30(const std::vector<bool, T1> &a0) {
  return a0.capacity();
}

template <typename T1> bool f31(const std::vector<bool, T1> &a0) {
  return a0.empty();
}

template <typename T1>
typename std::vector<bool, T1>::reference
f32(std::vector<bool, T1> &a0, typename std::vector<bool, T1>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_reference
f33(const std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1>
typename std::vector<bool, T1>::reference
f34(std::vector<bool, T1> &a0, typename std::vector<bool, T1>::size_type a1) {
  return a0.at(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_reference
f35(const std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::size_type a1) {
  return a0.at(a1);
}

template <typename T1>
void f36(std::vector<bool, T1> &a0,
         typename std::vector<bool, T1>::size_type a1) {
  return a0.reserve(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reference f37(std::vector<bool, T1> &a0) {
  return a0.front();
}

template <typename T1>
typename std::vector<bool, T1>::const_reference
f38(const std::vector<bool, T1> &a0) {
  return a0.front();
}

template <typename T1>
typename std::vector<bool, T1>::reference f39(std::vector<bool, T1> &a0) {
  return a0.back();
}

template <typename T1>
typename std::vector<bool, T1>::const_reference
f40(const std::vector<bool, T1> &a0) {
  return a0.back();
}

template <typename T1> void f41(std::vector<bool, T1> &a0, bool a1) {
  return a0.push_back(a1);
}

template <typename T1>
void f42(std::vector<bool, T1> &a0, std::vector<bool, T1> &a1) {
  return a0.swap(a1);
}

template <typename T1>
void f43(typename std::vector<bool, T1>::reference a0,
         typename std::vector<bool, T1>::reference a1) {
  return std::vector<bool, T1>::swap(a0, a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f44(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1, const bool &a2) {
  return a0.insert(a1, a2);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f45(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1,
    typename std::vector<bool, T1>::size_type a2, const bool &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f46(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1,
    std::initializer_list<bool> a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1> void f47(std::vector<bool, T1> &a0) {
  return a0.pop_back();
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f48(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f49(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1,
    typename std::vector<bool, T1>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1>
void f50(std::vector<bool, T1> &a0,
         typename std::vector<bool, T1>::size_type a1, bool a2) {
  return a0.resize(a1, a2);
}

template <typename T1> void f51(std::vector<bool, T1> &a0) {
  return a0.shrink_to_fit();
}

template <typename T1> void f52(std::vector<bool, T1> &a0) { return a0.flip(); }

template <typename T1> void f53(std::vector<bool, T1> &a0) {
  return a0.clear();
}

template <typename T1, typename T2>
std::vector<bool, T1>
f54(T2 a0, T2 a1, const typename std::vector<bool, T1>::allocator_type &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <typename T1, typename T2>
void f55(std::vector<bool, T1> &a0, T2 a1, T2 a2) {
  return a0.template assign<T2>(a1, a2);
}

template <typename T1, typename T2>
typename std::vector<bool, T1>::iterator
f56(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1, T2 a2, T2 a3) {
  return a0.template insert<T2>(a1, a2, a3);
}

template <typename T1, typename... T2>
void f57(std::vector<bool, T1> &a0, T2 &&...a1) {
  return a0.template emplace_back<T2...>(std::forward<T2>(a1)...);
}

template <typename T1, typename... T2>
typename std::vector<bool, T1>::iterator
f58(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1, T2 &&...a2) {
  return a0.template emplace<T2...>(a1, std::forward<T2>(a2)...);
}

template <typename T1>
typename std::vector<bool, T1>::reference
f59(const typename std::vector<bool, T1>::reference &a0) {
  return typename std::vector<bool, T1>::reference(a0);
}

template <typename T1>
bool f60(const typename std::vector<bool, T1>::reference &a0) {
  return static_cast<bool>(a0);
}

template <typename T1>
typename std::vector<bool, T1>::reference &
f61(typename std::vector<bool, T1>::reference &a0, bool a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reference &
f62(typename std::vector<bool, T1>::reference &a0,
    const typename std::vector<bool, T1>::reference &a1) {
  return a0.operator=(a1);
}

template <typename T1>
bool f63(const typename std::vector<bool, T1>::reference &a0,
         const typename std::vector<bool, T1>::reference &a1) {
  return a0.operator==(a1);
}

template <typename T1>
bool f64(const typename std::vector<bool, T1>::reference &a0,
         const typename std::vector<bool, T1>::reference &a1) {
  return a0.operator<(a1);
}

template <typename T1> void f65(typename std::vector<bool, T1>::reference &a0) {
  return a0.flip();
}

template <typename T1>
void f66(bool &a0, typename std::vector<bool, T1>::reference a1) {
  return swap(a0, a1);
}

template <typename T1>
void f67(typename std::vector<bool, T1>::reference a0, bool &a1) {
  return swap(a0, a1);
}

template <typename T1>
void f68(typename std::vector<bool, T1>::reference a0,
         typename std::vector<bool, T1>::reference a1) {
  return swap(a0, a1);
}

template <typename T1>
using t14 = typename std::vector<bool, T1>::iterator::reference;

template <typename T1>
using t15 = typename std::vector<bool, T1>::iterator::pointer;

template <typename T1>
using t16 = typename std::vector<bool, T1>::iterator::iterator;

template <typename T1> typename std::vector<bool, T1>::iterator f69() {
  return typename std::vector<bool, T1>::iterator();
}

template <typename T1>
typename std::vector<bool, T1>::iterator f70(unsigned long *a0,
                                             unsigned int a1) {
  return typename std::vector<bool, T1>::iterator(a0, a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f71(typename std::vector<bool, T1>::iterator &a0) {
  return a0.operator++();
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f72(typename std::vector<bool, T1>::iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f73(typename std::vector<bool, T1>::iterator &a0) {
  return a0.operator--();
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f74(typename std::vector<bool, T1>::iterator &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f75(typename std::vector<bool, T1>::iterator &a0,
    typename std::vector<bool, T1>::iterator::difference_type a1) {
  return a0.operator+=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f76(typename std::vector<bool, T1>::iterator &a0,
    typename std::vector<bool, T1>::iterator::difference_type a1) {
  return a0.operator-=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f77(const typename std::vector<bool, T1>::iterator &a0) {
  return typename std::vector<bool, T1>::iterator(a0);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f78(typename std::vector<bool, T1>::iterator &&a0) {
  return typename std::vector<bool, T1>::iterator(std::move(a0));
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f79(typename std::vector<bool, T1>::iterator &a0,
    const typename std::vector<bool, T1>::iterator &a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f80(typename std::vector<bool, T1>::iterator &a0,
    typename std::vector<bool, T1>::iterator &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
using t17 = typename std::vector<bool, T1>::const_iterator::reference;

template <typename T1>
using t18 = typename std::vector<bool, T1>::const_iterator::const_reference;

template <typename T1>
using t19 = typename std::vector<bool, T1>::const_iterator::pointer;

template <typename T1>
using t20 = typename std::vector<bool, T1>::const_iterator::const_iterator;

template <typename T1> typename std::vector<bool, T1>::const_iterator f81() {
  return typename std::vector<bool, T1>::const_iterator();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator f82(unsigned long *a0,
                                                   unsigned int a1) {
  return typename std::vector<bool, T1>::const_iterator(a0, a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f83(typename std::vector<bool, T1>::const_iterator &a0) {
  return a0.operator++();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f84(typename std::vector<bool, T1>::const_iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f85(typename std::vector<bool, T1>::const_iterator &a0) {
  return a0.operator--();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f86(typename std::vector<bool, T1>::const_iterator &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f87(typename std::vector<bool, T1>::const_iterator &a0,
    typename std::vector<bool, T1>::const_iterator::difference_type a1) {
  return a0.operator+=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f88(typename std::vector<bool, T1>::const_iterator &a0,
    typename std::vector<bool, T1>::const_iterator::difference_type a1) {
  return a0.operator-=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f89(const typename std::vector<bool, T1>::const_iterator &a0) {
  return typename std::vector<bool, T1>::const_iterator(a0);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f90(typename std::vector<bool, T1>::const_iterator &&a0) {
  return typename std::vector<bool, T1>::const_iterator(std::move(a0));
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f91(typename std::vector<bool, T1>::const_iterator &a0,
    const typename std::vector<bool, T1>::const_iterator &a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f92(typename std::vector<bool, T1>::const_iterator &a0,
    typename std::vector<bool, T1>::const_iterator &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
using t21 = typename std::vector<bool, T1>::const_reverse_iterator::pointer;

template <typename T1>
using t22 =
    typename std::vector<bool, T1>::const_reverse_iterator::difference_type;

template <typename T1>
using t23 = typename std::vector<bool, T1>::const_reverse_iterator::reference;

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator f93() {
  return typename std::vector<bool, T1>::const_reverse_iterator();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f94(const typename std::vector<bool, T1>::const_reverse_iterator &a0) {
  return typename std::vector<bool, T1>::const_reverse_iterator(a0);
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator &
f95(typename std::vector<bool, T1>::const_reverse_iterator &a0,
    const typename std::vector<bool, T1>::const_reverse_iterator &a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator::reference
f96(const typename std::vector<bool, T1>::const_reverse_iterator &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator::pointer
f97(const typename std::vector<bool, T1>::const_reverse_iterator &a0) {
  return a0.operator->();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator &
f98(typename std::vector<bool, T1>::const_reverse_iterator &a0) {
  return a0.operator++();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f99(typename std::vector<bool, T1>::const_reverse_iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator &
f100(typename std::vector<bool, T1>::const_reverse_iterator &a0) {
  return a0.operator--();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f101(typename std::vector<bool, T1>::const_reverse_iterator &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f102(const typename std::vector<bool, T1>::const_reverse_iterator &a0,
     typename std::vector<bool, T1>::const_reverse_iterator::difference_type
         a1) {
  return a0.operator+(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator &
f103(typename std::vector<bool, T1>::const_reverse_iterator &a0,
     typename std::vector<bool, T1>::const_reverse_iterator::difference_type
         a1) {
  return a0.operator+=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f104(const typename std::vector<bool, T1>::const_reverse_iterator &a0,
     typename std::vector<bool, T1>::const_reverse_iterator::difference_type
         a1) {
  return a0.operator-(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator &
f105(typename std::vector<bool, T1>::const_reverse_iterator &a0,
     typename std::vector<bool, T1>::const_reverse_iterator::difference_type
         a1) {
  return a0.operator-=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator::reference
f106(const typename std::vector<bool, T1>::const_reverse_iterator &a0,
     typename std::vector<bool, T1>::const_reverse_iterator::difference_type
         a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::vector<bool, T1>::const_reverse_iterator
f107(const std::reverse_iterator<T2> &a0) {
  return typename std::vector<bool, T1>::const_reverse_iterator(a0);
}

template <typename T1, typename T2>
typename std::vector<bool, T1>::const_reverse_iterator &
f108(typename std::vector<bool, T1>::const_reverse_iterator &a0,
     const std::reverse_iterator<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1>
using t24 = typename std::vector<bool, T1>::reverse_iterator::pointer;

template <typename T1>
using t25 = typename std::vector<bool, T1>::reverse_iterator::difference_type;

template <typename T1>
using t26 = typename std::vector<bool, T1>::reverse_iterator::reference;

template <typename T1> typename std::vector<bool, T1>::reverse_iterator f109() {
  return typename std::vector<bool, T1>::reverse_iterator();
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator
f110(const typename std::vector<bool, T1>::reverse_iterator &a0) {
  return typename std::vector<bool, T1>::reverse_iterator(a0);
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator &
f111(typename std::vector<bool, T1>::reverse_iterator &a0,
     const typename std::vector<bool, T1>::reverse_iterator &a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator::reference
f112(const typename std::vector<bool, T1>::reverse_iterator &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator::pointer
f113(const typename std::vector<bool, T1>::reverse_iterator &a0) {
  return a0.operator->();
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator &
f114(typename std::vector<bool, T1>::reverse_iterator &a0) {
  return a0.operator++();
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator
f115(typename std::vector<bool, T1>::reverse_iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator &
f116(typename std::vector<bool, T1>::reverse_iterator &a0) {
  return a0.operator--();
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator
f117(typename std::vector<bool, T1>::reverse_iterator &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator
f118(const typename std::vector<bool, T1>::reverse_iterator &a0,
     typename std::vector<bool, T1>::reverse_iterator::difference_type a1) {
  return a0.operator+(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator &
f119(typename std::vector<bool, T1>::reverse_iterator &a0,
     typename std::vector<bool, T1>::reverse_iterator::difference_type a1) {
  return a0.operator+=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator
f120(const typename std::vector<bool, T1>::reverse_iterator &a0,
     typename std::vector<bool, T1>::reverse_iterator::difference_type a1) {
  return a0.operator-(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator &
f121(typename std::vector<bool, T1>::reverse_iterator &a0,
     typename std::vector<bool, T1>::reverse_iterator::difference_type a1) {
  return a0.operator-=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator::reference
f122(const typename std::vector<bool, T1>::reverse_iterator &a0,
     typename std::vector<bool, T1>::reverse_iterator::difference_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::vector<bool, T1>::reverse_iterator
f123(const std::reverse_iterator<T2> &a0) {
  return typename std::vector<bool, T1>::reverse_iterator(a0);
}

template <typename T1, typename T2>
typename std::vector<bool, T1>::reverse_iterator &
f124(typename std::vector<bool, T1>::reverse_iterator &a0,
     const std::reverse_iterator<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2> using t27 = std::vector<T1, T2>;

template <typename T1, typename T2>
using t28 = typename std::vector<T1, T2>::value_type;

template <typename T1, typename T2>
using t29 = typename std::vector<T1, T2>::pointer;

template <typename T1, typename T2>
using t30 = typename std::vector<T1, T2>::const_pointer;

template <typename T1, typename T2>
using t31 = typename std::vector<T1, T2>::reference;

template <typename T1, typename T2>
using t32 = typename std::vector<T1, T2>::const_reference;

template <typename T1, typename T2>
using t33 = typename std::vector<T1, T2>::size_type;

template <typename T1, typename T2>
using t34 = typename std::vector<T1, T2>::difference_type;

template <typename T1, typename T2>
using t35 = typename std::vector<T1, T2>::allocator_type;

template <typename T1, typename T2> std::vector<T1, T2> f125() {
  return std::vector<T1, T2>();
}

template <typename T1, typename T2>
std::vector<T1, T2>
f126(const typename std::vector<T1, T2>::allocator_type &a0) {
  return std::vector<T1, T2>(a0);
}

template <typename T1, typename T2>
std::vector<T1, T2>
f127(typename std::vector<T1, T2>::size_type a0,
     const typename std::vector<T1, T2>::allocator_type &a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::vector<T1, T2>
f128(typename std::vector<T1, T2>::size_type a0,
     const typename std::vector<T1, T2>::value_type &a1,
     const typename std::vector<T1, T2>::allocator_type &a2) {
  return std::vector<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
std::vector<T1, T2> f129(const std::vector<T1, T2> &a0) {
  return std::vector<T1, T2>(a0);
}

template <typename T1, typename T2>
std::vector<T1, T2> f130(std::vector<T1, T2> &&a0) {
  return std::vector<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::vector<T1, T2>
f131(const std::vector<T1, T2> &a0,
     const typename std::vector<T1, T2>::allocator_type &a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::vector<T1, T2>
f132(std::vector<T1, T2> &&a0,
     const typename std::vector<T1, T2>::allocator_type &a1) {
  return std::vector<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::vector<T1, T2>
f133(std::initializer_list<typename std::vector<T1, T2>::value_type> a0,
     const typename std::vector<T1, T2>::allocator_type &a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> &f134(std::vector<T1, T2> &a0,
                          const std::vector<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> &f135(std::vector<T1, T2> &a0, std::vector<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::vector<T1, T2> &
f136(std::vector<T1, T2> &a0,
     std::initializer_list<typename std::vector<T1, T2>::value_type> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
void f137(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1,
          const typename std::vector<T1, T2>::value_type &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
void f138(std::vector<T1, T2> &a0,
          std::initializer_list<typename std::vector<T1, T2>::value_type> a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator f139(std::vector<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_iterator
f140(const std::vector<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator f141(std::vector<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_iterator
f142(const std::vector<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reverse_iterator f143(std::vector<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reverse_iterator
f144(const std::vector<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reverse_iterator f145(std::vector<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reverse_iterator
f146(const std::vector<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_iterator
f147(const std::vector<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_iterator
f148(const std::vector<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reverse_iterator
f149(const std::vector<T1, T2> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reverse_iterator
f150(const std::vector<T1, T2> &a0) {
  return a0.crend();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::size_type f151(const std::vector<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::size_type f152(const std::vector<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
void f153(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f154(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1,
          const typename std::vector<T1, T2>::value_type &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2> void f155(std::vector<T1, T2> &a0) {
  return a0.shrink_to_fit();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::size_type f156(const std::vector<T1, T2> &a0) {
  return a0.capacity();
}

template <typename T1, typename T2> bool f157(const std::vector<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
void f158(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.reserve(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference
f159(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f160(const std::vector<T1, T2> &a0,
     typename std::vector<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference
f161(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f162(const std::vector<T1, T2> &a0,
     typename std::vector<T1, T2>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f163(std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f164(const std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f165(std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f166(const std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2> T1 *f167(std::vector<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
const T1 *f168(const std::vector<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
void f169(std::vector<T1, T2> &a0,
          const typename std::vector<T1, T2>::value_type &a1) {
  return a0.push_back(a1);
}

template <typename T1, typename T2>
void f170(std::vector<T1, T2> &a0,
          typename std::vector<T1, T2>::value_type &&a1) {
  return a0.push_back(std::move(a1));
}

template <typename T1, typename T2> void f171(std::vector<T1, T2> &a0) {
  return a0.pop_back();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f172(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     const typename std::vector<T1, T2>::value_type &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f173(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     typename std::vector<T1, T2>::value_type &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f174(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     std::initializer_list<typename std::vector<T1, T2>::value_type> a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f175(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     typename std::vector<T1, T2>::size_type a2,
     const typename std::vector<T1, T2>::value_type &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f176(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f177(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     typename std::vector<T1, T2>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2>
void f178(std::vector<T1, T2> &a0, std::vector<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2> void f179(std::vector<T1, T2> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3>
std::vector<T1, T2>
f180(T3 a0, T3 a1, const typename std::vector<T1, T2>::allocator_type &a2) {
  return std::vector<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
void f181(std::vector<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(a1, a2);
}

template <typename T1, typename T2, typename... T3>
void f182(std::vector<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace_back<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename... T3>
typename std::vector<T1, T2>::iterator
f183(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     T3 &&...a2) {
  return a0.template emplace<T3...>(a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2, typename T3>
typename std::vector<T1, T2>::iterator
f184(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     T3 a2, T3 a3) {
  return a0.template insert<T3>(a1, a2, a3);
}

template <typename T1, typename T2>
using t36 = typename std::vector<T1, T2>::iterator::iterator_type;

template <typename T1, typename T2>
using t37 = typename std::vector<T1, T2>::iterator::iterator_category;

template <typename T1, typename T2>
using t38 = typename std::vector<T1, T2>::iterator::value_type;

template <typename T1, typename T2>
using t39 = typename std::vector<T1, T2>::iterator::difference_type;

template <typename T1, typename T2>
using t40 = typename std::vector<T1, T2>::iterator::reference;

template <typename T1, typename T2>
using t41 = typename std::vector<T1, T2>::iterator::pointer;

template <typename T1, typename T2>
using t42 = typename std::vector<T1, T2>::const_iterator::iterator_type;

template <typename T1, typename T2>
using t43 = typename std::vector<T1, T2>::const_iterator::iterator_category;

template <typename T1, typename T2>
using t44 = typename std::vector<T1, T2>::const_iterator::value_type;

template <typename T1, typename T2>
using t45 = typename std::vector<T1, T2>::const_iterator::difference_type;

template <typename T1, typename T2>
using t46 = typename std::vector<T1, T2>::const_iterator::reference;

template <typename T1, typename T2>
using t47 = typename std::vector<T1, T2>::const_iterator::pointer;

template <typename T1, typename T2>
using t48 = typename std::vector<T1, T2>::const_reverse_iterator::pointer;

template <typename T1, typename T2>
using t49 =
    typename std::vector<T1, T2>::const_reverse_iterator::difference_type;

template <typename T1, typename T2>
using t50 = typename std::vector<T1, T2>::const_reverse_iterator::reference;

template <typename T1, typename T2>
using t51 = typename std::vector<T1, T2>::reverse_iterator::pointer;

template <typename T1, typename T2>
using t52 = typename std::vector<T1, T2>::reverse_iterator::difference_type;

template <typename T1, typename T2>
using t53 = typename std::vector<T1, T2>::reverse_iterator::reference;
