#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::basic_string<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::basic_string<T1, T2, T3>::traits_type;

template <typename T1, typename T2, typename T3>
using t3 = typename std::basic_string<T1, T2, T3>::value_type;

template <typename T1, typename T2, typename T3>
using t4 = typename std::basic_string<T1, T2, T3>::allocator_type;

template <typename T1, typename T2, typename T3>
using t5 = typename std::basic_string<T1, T2, T3>::size_type;

template <typename T1, typename T2, typename T3>
using t6 = typename std::basic_string<T1, T2, T3>::difference_type;

template <typename T1, typename T2, typename T3>
using t7 = typename std::basic_string<T1, T2, T3>::reference;

template <typename T1, typename T2, typename T3>
using t8 = typename std::basic_string<T1, T2, T3>::const_reference;

template <typename T1, typename T2, typename T3>
using t9 = typename std::basic_string<T1, T2, T3>::pointer;

template <typename T1, typename T2, typename T3>
using t10 = typename std::basic_string<T1, T2, T3>::const_pointer;

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f1() {
  return std::basic_string<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f2(const T3 &a0) {
  return std::basic_string<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f3(const std::basic_string<T1, T2, T3> &a0) {
  return std::basic_string<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f4(const std::basic_string<T1, T2, T3> &a0,
   typename std::basic_string<T1, T2, T3>::size_type a1, const T3 &a2) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f5(const std::basic_string<T1, T2, T3> &a0,
   typename std::basic_string<T1, T2, T3>::size_type a1,
   typename std::basic_string<T1, T2, T3>::size_type a2) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f6(const std::basic_string<T1, T2, T3> &a0,
   typename std::basic_string<T1, T2, T3>::size_type a1,
   typename std::basic_string<T1, T2, T3>::size_type a2, const T3 &a3) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f7(const T1 *a0, typename std::basic_string<T1, T2, T3>::size_type a1,
   const T3 &a2) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f8(const T1 *a0, const T3 &a1) {
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f9(typename std::basic_string<T1, T2, T3>::size_type a0, T1 a1, const T3 &a2) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f10(std::basic_string<T1, T2, T3> &&a0) {
  return std::basic_string<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f11(std::initializer_list<T1> a0, const T3 &a1) {
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f12(const std::basic_string<T1, T2, T3> &a0,
                                  const T3 &a1) {
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f13(std::basic_string<T1, T2, T3> &&a0,
                                  const T3 &a1) {
  return std::basic_string<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f14(std::basic_string<T1, T2, T3> &a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f15(std::basic_string<T1, T2, T3> &a0,
                                   const T1 *a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f16(std::basic_string<T1, T2, T3> &a0, T1 a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f17(std::basic_string<T1, T2, T3> &a0,
                                   std::basic_string<T1, T2, T3> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f18(std::basic_string<T1, T2, T3> &a0,
                                   std::initializer_list<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f19(std::basic_string<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_iterator
f20(const std::basic_string<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f21(std::basic_string<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_iterator
f22(const std::basic_string<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reverse_iterator
f23(std::basic_string<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reverse_iterator
f24(const std::basic_string<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reverse_iterator
f25(std::basic_string<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reverse_iterator
f26(const std::basic_string<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_iterator
f27(const std::basic_string<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_iterator
f28(const std::basic_string<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reverse_iterator
f29(const std::basic_string<T1, T2, T3> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reverse_iterator
f30(const std::basic_string<T1, T2, T3> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f31(const std::basic_string<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f32(const std::basic_string<T1, T2, T3> &a0) {
  return a0.length();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f33(const std::basic_string<T1, T2, T3> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3>
void f34(std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1, T1 a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2, typename T3>
void f35(std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2, typename T3>
void f36(std::basic_string<T1, T2, T3> &a0) {
  return a0.shrink_to_fit();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f37(const std::basic_string<T1, T2, T3> &a0) {
  return a0.capacity();
}

template <typename T1, typename T2, typename T3>
void f38(std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.reserve(a1);
}

template <typename T1, typename T2, typename T3>
void f39(std::basic_string<T1, T2, T3> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3>
bool f40(const std::basic_string<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reference
f41(const std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reference
f42(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reference
f43(const std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reference
f44(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reference
f45(std::basic_string<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reference
f46(const std::basic_string<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reference
f47(std::basic_string<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reference
f48(const std::basic_string<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f49(std::basic_string<T1, T2, T3> &a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f50(std::basic_string<T1, T2, T3> &a0,
                                   const T1 *a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f51(std::basic_string<T1, T2, T3> &a0, T1 a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f52(std::basic_string<T1, T2, T3> &a0,
                                   std::initializer_list<T1> a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f53(std::basic_string<T1, T2, T3> &a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return a0.append(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f54(std::basic_string<T1, T2, T3> &a0, const std::basic_string<T1, T2, T3> &a1,
    typename std::basic_string<T1, T2, T3>::size_type a2,
    typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.append(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f55(std::basic_string<T1, T2, T3> &a0, const T1 *a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.append(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f56(std::basic_string<T1, T2, T3> &a0,
                                   const T1 *a1) {
  return a0.append(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f57(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1, T1 a2) {
  return a0.append(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f58(std::basic_string<T1, T2, T3> &a0,
                                   std::initializer_list<T1> a1) {
  return a0.append(a1);
}

template <typename T1, typename T2, typename T3>
void f59(std::basic_string<T1, T2, T3> &a0, T1 a1) {
  return a0.push_back(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f60(std::basic_string<T1, T2, T3> &a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f61(std::basic_string<T1, T2, T3> &a0,
                                   std::basic_string<T1, T2, T3> &&a1) {
  return a0.assign(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f62(std::basic_string<T1, T2, T3> &a0, const std::basic_string<T1, T2, T3> &a1,
    typename std::basic_string<T1, T2, T3>::size_type a2,
    typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.assign(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f63(std::basic_string<T1, T2, T3> &a0, const T1 *a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f64(std::basic_string<T1, T2, T3> &a0,
                                   const T1 *a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f65(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1, T1 a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f66(std::basic_string<T1, T2, T3> &a0,
                                   std::initializer_list<T1> a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f67(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::size_type a2, T1 a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f68(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    std::initializer_list<T1> a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f69(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    const std::basic_string<T1, T2, T3> &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f70(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    const std::basic_string<T1, T2, T3> &a2,
    typename std::basic_string<T1, T2, T3>::size_type a3,
    typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.insert(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f71(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1, const T1 *a2,
    typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f72(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1, const T1 *a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f73(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2, T1 a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f74(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1, T1 a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f75(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f76(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f77(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2, typename T3>
void f78(std::basic_string<T1, T2, T3> &a0) {
  return a0.pop_back();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f79(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2,
    const std::basic_string<T1, T2, T3> &a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f80(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2,
    const std::basic_string<T1, T2, T3> &a3,
    typename std::basic_string<T1, T2, T3>::size_type a4,
    typename std::basic_string<T1, T2, T3>::size_type a5) {
  return a0.replace(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f81(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2, const T1 *a3,
    typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f82(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2, const T1 *a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f83(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2,
    typename std::basic_string<T1, T2, T3>::size_type a3, T1 a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f84(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2,
    const std::basic_string<T1, T2, T3> &a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f85(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2, const T1 *a3,
    typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f86(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2, const T1 *a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f87(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2,
    typename std::basic_string<T1, T2, T3>::size_type a3, T1 a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f88(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2, T1 *a3, T1 *a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f89(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2, const T1 *a3,
    const T1 *a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f90(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2,
    typename std::basic_string<T1, T2, T3>::iterator a3,
    typename std::basic_string<T1, T2, T3>::iterator a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f91(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2,
    typename std::basic_string<T1, T2, T3>::const_iterator a3,
    typename std::basic_string<T1, T2, T3>::const_iterator a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f92(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2,
    std::initializer_list<T1> a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f93(const std::basic_string<T1, T2, T3> &a0, T1 *a1,
    typename std::basic_string<T1, T2, T3>::size_type a2,
    typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.copy(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
void f94(std::basic_string<T1, T2, T3> &a0, std::basic_string<T1, T2, T3> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
const T1 *f95(const std::basic_string<T1, T2, T3> &a0) {
  return a0.c_str();
}

template <typename T1, typename T2, typename T3>
const T1 *f96(const std::basic_string<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::allocator_type
f97(const std::basic_string<T1, T2, T3> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f98(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
    typename std::basic_string<T1, T2, T3>::size_type a2,
    typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f99(const std::basic_string<T1, T2, T3> &a0,
    const std::basic_string<T1, T2, T3> &a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f100(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f101(const std::basic_string<T1, T2, T3> &a0, T1 a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f102(const std::basic_string<T1, T2, T3> &a0,
     const std::basic_string<T1, T2, T3> &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f103(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.rfind(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f104(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f105(const std::basic_string<T1, T2, T3> &a0, T1 a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f106(const std::basic_string<T1, T2, T3> &a0,
     const std::basic_string<T1, T2, T3> &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f107(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_first_of(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f108(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f109(const std::basic_string<T1, T2, T3> &a0, T1 a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f110(const std::basic_string<T1, T2, T3> &a0,
     const std::basic_string<T1, T2, T3> &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f111(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_last_of(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f112(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f113(const std::basic_string<T1, T2, T3> &a0, T1 a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f114(const std::basic_string<T1, T2, T3> &a0,
     const std::basic_string<T1, T2, T3> &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f115(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_first_not_of(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f116(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f117(const std::basic_string<T1, T2, T3> &a0, T1 a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f118(const std::basic_string<T1, T2, T3> &a0,
     const std::basic_string<T1, T2, T3> &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f119(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_last_not_of(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f120(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f121(const std::basic_string<T1, T2, T3> &a0, T1 a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f122(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.substr(a1, a2);
}

template <typename T1, typename T2, typename T3>
int f123(const std::basic_string<T1, T2, T3> &a0,
         const std::basic_string<T1, T2, T3> &a1) {
  return a0.compare(a1);
}

template <typename T1, typename T2, typename T3>
int f124(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2,
         const std::basic_string<T1, T2, T3> &a3) {
  return a0.compare(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
int f125(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2,
         const std::basic_string<T1, T2, T3> &a3,
         typename std::basic_string<T1, T2, T3>::size_type a4,
         typename std::basic_string<T1, T2, T3>::size_type a5) {
  return a0.compare(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3>
int f126(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0.compare(a1);
}

template <typename T1, typename T2, typename T3>
int f127(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2, const T1 *a3) {
  return a0.compare(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
int f128(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2, const T1 *a3,
         typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.compare(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> f129(T4 a0, T4 a1, const T3 &a2) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &f130(std::basic_string<T1, T2, T3> &a0, T4 a1,
                                    T4 a2) {
  return a0.template append<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &f131(std::basic_string<T1, T2, T3> &a0, T4 a1,
                                    T4 a2) {
  return a0.template assign<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_string<T1, T2, T3>::iterator
f132(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1, T4 a2, T4 a3) {
  return a0.template insert<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &
f133(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, T4 a3, T4 a4) {
  return a0.template replace<T4>(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
using t11 = typename std::basic_string<T1, T2, T3>::iterator::iterator_type;

template <typename T1, typename T2, typename T3>
using t12 = typename std::basic_string<T1, T2, T3>::iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t13 = typename std::basic_string<T1, T2, T3>::iterator::value_type;

template <typename T1, typename T2, typename T3>
using t14 = typename std::basic_string<T1, T2, T3>::iterator::difference_type;

template <typename T1, typename T2, typename T3>
using t15 = typename std::basic_string<T1, T2, T3>::iterator::reference;

template <typename T1, typename T2, typename T3>
using t16 = typename std::basic_string<T1, T2, T3>::iterator::pointer;

template <typename T1, typename T2, typename T3>
using t17 =
    typename std::basic_string<T1, T2, T3>::const_iterator::iterator_type;

template <typename T1, typename T2, typename T3>
using t18 =
    typename std::basic_string<T1, T2, T3>::const_iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t19 = typename std::basic_string<T1, T2, T3>::const_iterator::value_type;

template <typename T1, typename T2, typename T3>
using t20 =
    typename std::basic_string<T1, T2, T3>::const_iterator::difference_type;

template <typename T1, typename T2, typename T3>
using t21 = typename std::basic_string<T1, T2, T3>::const_iterator::reference;

template <typename T1, typename T2, typename T3>
using t22 = typename std::basic_string<T1, T2, T3>::const_iterator::pointer;

template <typename T1, typename T2, typename T3>
using t23 =
    typename std::basic_string<T1, T2, T3>::const_reverse_iterator::pointer;

template <typename T1, typename T2, typename T3>
using t24 =
    typename std::basic_string<T1, T2,
                               T3>::const_reverse_iterator::difference_type;

template <typename T1, typename T2, typename T3>
using t25 =
    typename std::basic_string<T1, T2, T3>::const_reverse_iterator::reference;

template <typename T1, typename T2, typename T3>
using t26 = typename std::basic_string<T1, T2, T3>::reverse_iterator::pointer;

template <typename T1, typename T2, typename T3>
using t27 =
    typename std::basic_string<T1, T2, T3>::reverse_iterator::difference_type;

template <typename T1, typename T2, typename T3>
using t28 = typename std::basic_string<T1, T2, T3>::reverse_iterator::reference;

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type f134() {
  return std::basic_string<T1, T2, T3>::npos;
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f135(std::basic_istream<T1, T2> &&a0,
                                 std::basic_string<T1, T2, T3> &a1) {
  return std::getline<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f136(std::basic_istream<T1, T2> &&a0,
                                 std::basic_string<T1, T2, T3> &a1, T1 a2) {
  return std::getline<T1, T2, T3>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f137(std::basic_istream<T1, T2> &a0,
                                 std::basic_string<T1, T2, T3> &a1) {
  return std::getline<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f138(std::basic_istream<T1, T2> &a0,
                                 std::basic_string<T1, T2, T3> &a1, T1 a2) {
  return std::getline<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f139(std::basic_string<T1, T2, T3> &&a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return std::move(a0) + a1;
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f140(std::basic_string<T1, T2, T3> &&a0,
                                   const T1 *a1) {
  return std::move(a0) + a1;
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f141(std::basic_string<T1, T2, T3> &&a0,
                                   std::basic_string<T1, T2, T3> &&a1) {
  return std::move(a0) + std::move(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f142(std::basic_string<T1, T2, T3> &&a0, T1 a1) {
  return std::move(a0) + a1;
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f143(const std::basic_string<T1, T2, T3> &a0,
                                   std::basic_string<T1, T2, T3> &&a1) {
  return a0 + std::move(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f144(const std::basic_string<T1, T2, T3> &a0,
                                   const T1 *a1) {
  return a0 + a1;
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f145(const std::basic_string<T1, T2, T3> &a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return a0 + a1;
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f146(const std::basic_string<T1, T2, T3> &a0,
                                   T1 a1) {
  return a0 + a1;
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f147(const T1 *a0,
                                   std::basic_string<T1, T2, T3> &&a1) {
  return a0 + std::move(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f148(const T1 *a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return a0 + a1;
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f149(T1 a0, std::basic_string<T1, T2, T3> &&a1) {
  return a0 + std::move(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f150(T1 a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return a0 + a1;
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f151(std::basic_ostream<T1, T2> &a0,
                                 const std::basic_string<T1, T2, T3> &a1) {
  return a0 << a1;
}

template <typename T1, typename T2, typename T3>
bool f152(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3>
bool f153(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f154(std::basic_istream<T1, T2> &a0,
                                 std::basic_string<T1, T2, T3> &a1) {
  return a0 >> a1;
}

double f155(const std::string &a0, std::size_t *a1) {
  return std::stod(a0, a1);
}

double f156(const std::wstring &a0, std::size_t *a1) {
  return std::stod(a0, a1);
}

float f157(const std::string &a0, std::size_t *a1) { return std::stof(a0, a1); }

float f158(const std::wstring &a0, std::size_t *a1) {
  return std::stof(a0, a1);
}

int f159(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoi(a0, a1, a2);
}

int f160(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoi(a0, a1, a2);
}

long f161(const std::string &a0, std::size_t *a1, int a2) {
  return std::stol(a0, a1, a2);
}

long f162(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stol(a0, a1, a2);
}

long double f163(const std::string &a0, std::size_t *a1) {
  return std::stold(a0, a1);
}

long double f164(const std::wstring &a0, std::size_t *a1) {
  return std::stold(a0, a1);
}

long long f165(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoll(a0, a1, a2);
}

long long f166(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoll(a0, a1, a2);
}

unsigned long f167(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoul(a0, a1, a2);
}

unsigned long f168(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoul(a0, a1, a2);
}

unsigned long long f169(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoull(a0, a1, a2);
}

unsigned long long f170(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoull(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
void f171(std::basic_string<T1, T2, T3> &a0,
          std::basic_string<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}

std::string f172(long double a0) { return std::to_string(a0); }

std::string f173(int a0) { return std::to_string(a0); }

std::string f174(long long a0) { return std::to_string(a0); }

std::string f175(long a0) { return std::to_string(a0); }

std::string f176(double a0) { return std::to_string(a0); }

std::string f177(float a0) { return std::to_string(a0); }

std::string f178(unsigned int a0) { return std::to_string(a0); }

std::string f179(unsigned long long a0) { return std::to_string(a0); }

std::string f180(unsigned long a0) { return std::to_string(a0); }

std::wstring f181(long double a0) { return std::to_wstring(a0); }

std::wstring f182(int a0) { return std::to_wstring(a0); }

std::wstring f183(long long a0) { return std::to_wstring(a0); }

std::wstring f184(long a0) { return std::to_wstring(a0); }

std::wstring f185(double a0) { return std::to_wstring(a0); }

std::wstring f186(float a0) { return std::to_wstring(a0); }

std::wstring f187(unsigned int a0) { return std::to_wstring(a0); }

std::wstring f188(unsigned long long a0) { return std::to_wstring(a0); }

std::wstring f189(unsigned long a0) { return std::to_wstring(a0); }
