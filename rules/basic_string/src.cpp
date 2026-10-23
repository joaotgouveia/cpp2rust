#include <algorithm>
#include <initializer_list>
#include <string>
#include <string_view>
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
   typename std::basic_string<T1, T2, T3>::size_type a2, const T3 &a3) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f6(const T1 *a0, typename std::basic_string<T1, T2, T3>::size_type a1,
   const T3 &a2) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f7(std::basic_string<T1, T2, T3> &&a0) {
  return std::basic_string<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f8(std::initializer_list<T1> a0, const T3 &a1) {
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f9(const std::basic_string<T1, T2, T3> &a0,
                                 const T3 &a1) {
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f10(std::basic_string<T1, T2, T3> &&a0,
                                  const T3 &a1) {
  return std::basic_string<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f11(std::basic_string<T1, T2, T3> &a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f12(std::basic_string<T1, T2, T3> &a0,
                                   const T1 *a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f13(std::basic_string<T1, T2, T3> &a0, T1 a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f14(std::basic_string<T1, T2, T3> &a0,
                                   std::basic_string<T1, T2, T3> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f15(std::basic_string<T1, T2, T3> &a0,
                                   std::initializer_list<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f16(std::basic_string<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_iterator
f17(const std::basic_string<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f18(std::basic_string<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_iterator
f19(const std::basic_string<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reverse_iterator
f20(std::basic_string<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reverse_iterator
f21(const std::basic_string<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reverse_iterator
f22(std::basic_string<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reverse_iterator
f23(const std::basic_string<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_iterator
f24(const std::basic_string<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_iterator
f25(const std::basic_string<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reverse_iterator
f26(const std::basic_string<T1, T2, T3> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reverse_iterator
f27(const std::basic_string<T1, T2, T3> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f28(const std::basic_string<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f29(const std::basic_string<T1, T2, T3> &a0) {
  return a0.length();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f30(const std::basic_string<T1, T2, T3> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3>
void f31(std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1, T1 a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2, typename T3>
void f32(std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2, typename T3>
void f33(std::basic_string<T1, T2, T3> &a0) {
  return a0.shrink_to_fit();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f34(const std::basic_string<T1, T2, T3> &a0) {
  return a0.capacity();
}

template <typename T1, typename T2, typename T3>
void f35(std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.reserve(a1);
}

template <typename T1, typename T2, typename T3>
void f36(std::basic_string<T1, T2, T3> &a0) {
  return a0.reserve();
}

template <typename T1, typename T2, typename T3>
void f37(std::basic_string<T1, T2, T3> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3>
bool f38(const std::basic_string<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f39(std::basic_string<T1, T2, T3> &a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f40(std::basic_string<T1, T2, T3> &a0,
                                   const T1 *a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f41(std::basic_string<T1, T2, T3> &a0, T1 a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f42(std::basic_string<T1, T2, T3> &a0,
                                   std::initializer_list<T1> a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f43(std::basic_string<T1, T2, T3> &a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return a0.append(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f44(std::basic_string<T1, T2, T3> &a0, const std::basic_string<T1, T2, T3> &a1,
    typename std::basic_string<T1, T2, T3>::size_type a2,
    typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.append(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f45(std::basic_string<T1, T2, T3> &a0, const T1 *a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.append(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f46(std::basic_string<T1, T2, T3> &a0,
                                   const T1 *a1) {
  return a0.append(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f47(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1, T1 a2) {
  return a0.append(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f48(std::basic_string<T1, T2, T3> &a0,
                                   std::initializer_list<T1> a1) {
  return a0.append(a1);
}

template <typename T1, typename T2, typename T3>
void f49(std::basic_string<T1, T2, T3> &a0, T1 a1) {
  return a0.push_back(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f50(std::basic_string<T1, T2, T3> &a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f51(std::basic_string<T1, T2, T3> &a0,
                                   std::basic_string<T1, T2, T3> &&a1) {
  return a0.assign(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f52(std::basic_string<T1, T2, T3> &a0, const std::basic_string<T1, T2, T3> &a1,
    typename std::basic_string<T1, T2, T3>::size_type a2,
    typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.assign(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f53(std::basic_string<T1, T2, T3> &a0, const T1 *a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f54(std::basic_string<T1, T2, T3> &a0,
                                   const T1 *a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f55(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1, T1 a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f56(std::basic_string<T1, T2, T3> &a0,
                                   std::initializer_list<T1> a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f57(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::size_type a2, T1 a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f58(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    std::initializer_list<T1> a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f59(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    const std::basic_string<T1, T2, T3> &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f60(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    const std::basic_string<T1, T2, T3> &a2,
    typename std::basic_string<T1, T2, T3>::size_type a3,
    typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.insert(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f61(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1, const T1 *a2,
    typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f62(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1, const T1 *a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f63(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2, T1 a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f64(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1, T1 a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f65(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f66(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f67(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2, typename T3>
void f68(std::basic_string<T1, T2, T3> &a0) {
  return a0.pop_back();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f69(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2,
    const std::basic_string<T1, T2, T3> &a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f70(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2,
    const std::basic_string<T1, T2, T3> &a3,
    typename std::basic_string<T1, T2, T3>::size_type a4,
    typename std::basic_string<T1, T2, T3>::size_type a5) {
  return a0.replace(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f71(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2, const T1 *a3,
    typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f72(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2, const T1 *a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f73(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2,
    typename std::basic_string<T1, T2, T3>::size_type a3, T1 a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f74(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2,
    const std::basic_string<T1, T2, T3> &a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f75(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2, const T1 *a3,
    typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f76(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2, const T1 *a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f77(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2,
    typename std::basic_string<T1, T2, T3>::size_type a3, T1 a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f78(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2,
    std::initializer_list<T1> a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f79(const std::basic_string<T1, T2, T3> &a0, T1 *a1,
    typename std::basic_string<T1, T2, T3>::size_type a2,
    typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.copy(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
void f80(std::basic_string<T1, T2, T3> &a0, std::basic_string<T1, T2, T3> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
const T1 *f81(const std::basic_string<T1, T2, T3> &a0) {
  return a0.c_str();
}

template <typename T1, typename T2, typename T3>
const T1 *f82(const std::basic_string<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f83(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
    typename std::basic_string<T1, T2, T3>::size_type a2,
    typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f84(const std::basic_string<T1, T2, T3> &a0,
    const std::basic_string<T1, T2, T3> &a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f85(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f86(const std::basic_string<T1, T2, T3> &a0, T1 a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f87(const std::basic_string<T1, T2, T3> &a0,
    const std::basic_string<T1, T2, T3> &a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f88(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
    typename std::basic_string<T1, T2, T3>::size_type a2,
    typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.rfind(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f89(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f90(const std::basic_string<T1, T2, T3> &a0, T1 a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f91(const std::basic_string<T1, T2, T3> &a0,
    const std::basic_string<T1, T2, T3> &a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f92(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
    typename std::basic_string<T1, T2, T3>::size_type a2,
    typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_first_of(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f93(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f94(const std::basic_string<T1, T2, T3> &a0, T1 a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f95(const std::basic_string<T1, T2, T3> &a0,
    const std::basic_string<T1, T2, T3> &a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f96(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
    typename std::basic_string<T1, T2, T3>::size_type a2,
    typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_last_of(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f97(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f98(const std::basic_string<T1, T2, T3> &a0, T1 a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f99(const std::basic_string<T1, T2, T3> &a0,
    const std::basic_string<T1, T2, T3> &a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f100(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_first_not_of(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f101(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f102(const std::basic_string<T1, T2, T3> &a0, T1 a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f103(const std::basic_string<T1, T2, T3> &a0,
     const std::basic_string<T1, T2, T3> &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f104(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_last_not_of(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f105(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f106(const std::basic_string<T1, T2, T3> &a0, T1 a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f107(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.substr(a1, a2);
}

template <typename T1, typename T2, typename T3>
int f108(const std::basic_string<T1, T2, T3> &a0,
         const std::basic_string<T1, T2, T3> &a1) {
  return a0.compare(a1);
}

template <typename T1, typename T2, typename T3>
int f109(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2,
         const std::basic_string<T1, T2, T3> &a3) {
  return a0.compare(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
int f110(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2,
         const std::basic_string<T1, T2, T3> &a3,
         typename std::basic_string<T1, T2, T3>::size_type a4,
         typename std::basic_string<T1, T2, T3>::size_type a5) {
  return a0.compare(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3>
int f111(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0.compare(a1);
}

template <typename T1, typename T2, typename T3>
int f112(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2, const T1 *a3) {
  return a0.compare(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
int f113(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2, const T1 *a3,
         typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.compare(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
using t11 = typename std::basic_string<T1, T2, T3>::iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t12 = typename std::basic_string<T1, T2, T3>::iterator::value_type;

template <typename T1, typename T2, typename T3>
using t13 = typename std::basic_string<T1, T2, T3>::iterator::difference_type;

template <typename T1, typename T2, typename T3>
using t14 = typename std::basic_string<T1, T2, T3>::iterator::reference;

template <typename T1, typename T2, typename T3>
using t15 = typename std::basic_string<T1, T2, T3>::iterator::pointer;

template <typename T1, typename T2, typename T3>
using t16 =
    typename std::basic_string<T1, T2, T3>::const_iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t17 = typename std::basic_string<T1, T2, T3>::const_iterator::value_type;

template <typename T1, typename T2, typename T3>
using t18 =
    typename std::basic_string<T1, T2, T3>::const_iterator::difference_type;

template <typename T1, typename T2, typename T3>
using t19 = typename std::basic_string<T1, T2, T3>::const_iterator::reference;

template <typename T1, typename T2, typename T3>
using t20 = typename std::basic_string<T1, T2, T3>::const_iterator::pointer;

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type f114() {
  return std::basic_string<T1, T2, T3>::npos;
}
