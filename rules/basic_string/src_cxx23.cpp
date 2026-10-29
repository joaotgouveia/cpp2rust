#include <algorithm>
#include <initializer_list>
#include <iosfwd>
#include <string>
#include <utility>

std::string f1(long double a0) { return std::to_string(a0); }

std::string f2(int a0) { return std::to_string(a0); }

std::string f3(long long a0) { return std::to_string(a0); }

std::string f4(long a0) { return std::to_string(a0); }

std::string f5(double a0) { return std::to_string(a0); }

std::string f6(float a0) { return std::to_string(a0); }

std::string f7(unsigned int a0) { return std::to_string(a0); }

std::string f8(unsigned long long a0) { return std::to_string(a0); }

std::string f9(unsigned long a0) { return std::to_string(a0); }

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
using t11 = typename std::basic_string<T1, T2, T3>::iterator;

template <typename T1, typename T2, typename T3>
using t12 = typename std::basic_string<T1, T2, T3>::const_iterator;

template <typename T1, typename T2, typename T3>
using t13 = typename std::basic_string<T1, T2, T3>::const_reverse_iterator;

template <typename T1, typename T2, typename T3>
using t14 = typename std::basic_string<T1, T2, T3>::reverse_iterator;

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f10()
  requires(std::is_default_constructible_v<T3>)
{
  return std::basic_string<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f11(const T3 &a0) {
  return std::basic_string<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f12(const std::basic_string<T1, T2, T3> &a0) {
  return std::basic_string<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f13(const std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1, const T3 &a2) {
  return std::basic_string<T1, T2, T3>(
      a0, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f14(const std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return std::basic_string<T1, T2, T3>(
      a0, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f15(const std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2, const T3 &a3) {
  return std::basic_string<T1, T2, T3>(
      a0, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2), a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f16(const T1 *a0, typename std::basic_string<T1, T2, T3>::size_type a1,
    const T3 &a2) {
  return std::basic_string<T1, T2, T3>(
      a0, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f17(std::basic_string<T1, T2, T3> &&a0) {
  return std::basic_string<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f18(std::initializer_list<T1> a0, const T3 &a1) {
  return std::basic_string<T1, T2, T3>(
      std::forward<std::initializer_list<T1>>(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f19(const std::basic_string<T1, T2, T3> &a0,
                                  const T3 &a1) {
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f20(std::basic_string<T1, T2, T3> &&a0,
                                  const T3 &a1) {
  return std::basic_string<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f21(std::basic_string<T1, T2, T3> &a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f22(std::basic_string<T1, T2, T3> &a0,
                                   const T1 *a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f23(std::basic_string<T1, T2, T3> &a0, T1 a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f24(std::basic_string<T1, T2, T3> &a0,
                                   std::basic_string<T1, T2, T3> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f25(std::basic_string<T1, T2, T3> &a0,
                                   std::initializer_list<T1> a1) {
  return a0.operator=(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string_view<T1, T2> f26(const std::basic_string<T1, T2, T3> &a0) {
  return static_cast<std::basic_string_view<T1, T2>>(a0);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f27(std::basic_string<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_iterator
f28(const std::basic_string<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f29(std::basic_string<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_iterator
f30(const std::basic_string<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reverse_iterator
f31(std::basic_string<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reverse_iterator
f32(const std::basic_string<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reverse_iterator
f33(std::basic_string<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reverse_iterator
f34(const std::basic_string<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_iterator
f35(const std::basic_string<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_iterator
f36(const std::basic_string<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reverse_iterator
f37(const std::basic_string<T1, T2, T3> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reverse_iterator
f38(const std::basic_string<T1, T2, T3> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f39(const std::basic_string<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f40(const std::basic_string<T1, T2, T3> &a0) {
  return a0.length();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f41(const std::basic_string<T1, T2, T3> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3>
void f42(std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1, T1 a2) {
  return a0.resize(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f43(std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.resize(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1));
}

template <typename T1, typename T2, typename T3>
void f44(std::basic_string<T1, T2, T3> &a0) {
  return a0.shrink_to_fit();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f45(const std::basic_string<T1, T2, T3> &a0) {
  return a0.capacity();
}

template <typename T1, typename T2, typename T3>
void f46(std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.reserve(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1));
}

template <typename T1, typename T2, typename T3>
void f47(std::basic_string<T1, T2, T3> &a0) {
  return a0.reserve();
}

template <typename T1, typename T2, typename T3>
void f48(std::basic_string<T1, T2, T3> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3>
bool f49(const std::basic_string<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reference
f50(const std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.operator[](
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reference
f51(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.operator[](
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reference
f52(const std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.at(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reference
f53(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.at(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reference
f54(std::basic_string<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reference
f55(const std::basic_string<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reference
f56(std::basic_string<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reference
f57(const std::basic_string<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f58(std::basic_string<T1, T2, T3> &a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f59(std::basic_string<T1, T2, T3> &a0,
                                   const T1 *a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f60(std::basic_string<T1, T2, T3> &a0, T1 a1) {
  return a0.operator+=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f61(std::basic_string<T1, T2, T3> &a0,
                                   std::initializer_list<T1> a1) {
  return a0.operator+=(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f62(std::basic_string<T1, T2, T3> &a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return a0.append(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f63(std::basic_string<T1, T2, T3> &a0, const std::basic_string<T1, T2, T3> &a1,
    typename std::basic_string<T1, T2, T3>::size_type a2,
    typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.append(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a3));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f64(std::basic_string<T1, T2, T3> &a0, const T1 *a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.append(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f65(std::basic_string<T1, T2, T3> &a0,
                                   const T1 *a1) {
  return a0.append(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f66(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1, T1 a2) {
  return a0.append(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f67(std::basic_string<T1, T2, T3> &a0,
                                   std::initializer_list<T1> a1) {
  return a0.append(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2, typename T3>
void f68(std::basic_string<T1, T2, T3> &a0, T1 a1) {
  return a0.push_back(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f69(std::basic_string<T1, T2, T3> &a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f70(std::basic_string<T1, T2, T3> &a0,
                                   std::basic_string<T1, T2, T3> &&a1) {
  return a0.assign(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f71(std::basic_string<T1, T2, T3> &a0, const std::basic_string<T1, T2, T3> &a1,
    typename std::basic_string<T1, T2, T3>::size_type a2,
    typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.assign(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a3));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f72(std::basic_string<T1, T2, T3> &a0, const T1 *a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.assign(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f73(std::basic_string<T1, T2, T3> &a0,
                                   const T1 *a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f74(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1, T1 a2) {
  return a0.assign(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f75(std::basic_string<T1, T2, T3> &a0,
                                   std::initializer_list<T1> a1) {
  return a0.assign(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f76(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::size_type a2, T1 a3) {
  return a0.insert(
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2),
      std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f77(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    std::initializer_list<T1> a2) {
  return a0.insert(
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a1),
      std::forward<std::initializer_list<T1>>(a2));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f78(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    const std::basic_string<T1, T2, T3> &a2) {
  return a0.insert(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1), a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f79(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    const std::basic_string<T1, T2, T3> &a2,
    typename std::basic_string<T1, T2, T3>::size_type a3,
    typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.insert(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1), a2,
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a3),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a4));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f80(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1, const T1 *a2,
    typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.insert(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1), a2,
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a3));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f81(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1, const T1 *a2) {
  return a0.insert(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1), a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f82(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2, T1 a3) {
  return a0.insert(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2),
      std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f83(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1, T1 a2) {
  return a0.insert(
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f84(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.erase(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f85(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1) {
  return a0.erase(
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a1));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f86(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2) {
  return a0.erase(
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a2));
}

template <typename T1, typename T2, typename T3>
void f87(std::basic_string<T1, T2, T3> &a0) {
  return a0.pop_back();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f88(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2,
    const std::basic_string<T1, T2, T3> &a3) {
  return a0.replace(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2), a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f89(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2,
    const std::basic_string<T1, T2, T3> &a3,
    typename std::basic_string<T1, T2, T3>::size_type a4,
    typename std::basic_string<T1, T2, T3>::size_type a5) {
  return a0.replace(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2), a3,
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a4),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a5));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f90(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2, const T1 *a3,
    typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.replace(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2), a3,
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a4));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f91(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2, const T1 *a3) {
  return a0.replace(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2), a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f92(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2,
    typename std::basic_string<T1, T2, T3>::size_type a3, T1 a4) {
  return a0.replace(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a3),
      std::move(a4));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f93(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2,
    const std::basic_string<T1, T2, T3> &a3) {
  return a0.replace(
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a2),
      a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f94(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2, const T1 *a3,
    typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.replace(
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a2),
      a3, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a4));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f95(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2, const T1 *a3) {
  return a0.replace(
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a2),
      a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f96(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2,
    typename std::basic_string<T1, T2, T3>::size_type a3, T1 a4) {
  return a0.replace(
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a2),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a3),
      std::move(a4));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f97(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2, T1 *a3, T1 *a4) {
  return a0.replace(
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a2),
      a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f98(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2, const T1 *a3,
    const T1 *a4) {
  return a0.replace(
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a2),
      a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f99(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2,
    typename std::basic_string<T1, T2, T3>::iterator a3,
    typename std::basic_string<T1, T2, T3>::iterator a4) {
  return a0.replace(
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a2),
      std::forward<typename std::basic_string<T1, T2, T3>::iterator>(a3),
      std::forward<typename std::basic_string<T1, T2, T3>::iterator>(a4));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f100(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2,
     typename std::basic_string<T1, T2, T3>::const_iterator a3,
     typename std::basic_string<T1, T2, T3>::const_iterator a4) {
  return a0.replace(
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a2),
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a3),
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a4));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f101(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2,
     std::initializer_list<T1> a3) {
  return a0.replace(
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a2),
      std::forward<std::initializer_list<T1>>(a3));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f102(const std::basic_string<T1, T2, T3> &a0, T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.copy(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a3));
}

template <typename T1, typename T2, typename T3>
void f103(std::basic_string<T1, T2, T3> &a0,
          std::basic_string<T1, T2, T3> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
const T1 *f104(const std::basic_string<T1, T2, T3> &a0) {
  return a0.c_str();
}

template <typename T1, typename T2, typename T3>
const T1 *f105(const std::basic_string<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, typename T3>
T1 *f106(std::basic_string<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, typename T3>
T3 f107(const std::basic_string<T1, T2, T3> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f108(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a3));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f109(const std::basic_string<T1, T2, T3> &a0,
     const std::basic_string<T1, T2, T3> &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f110(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f111(const std::basic_string<T1, T2, T3> &a0, T1 a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find(
      std::move(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f112(const std::basic_string<T1, T2, T3> &a0,
     const std::basic_string<T1, T2, T3> &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.rfind(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f113(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.rfind(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a3));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f114(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.rfind(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f115(const std::basic_string<T1, T2, T3> &a0, T1 a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.rfind(
      std::move(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f116(const std::basic_string<T1, T2, T3> &a0,
     const std::basic_string<T1, T2, T3> &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_of(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f117(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_first_of(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a3));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f118(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_of(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f119(const std::basic_string<T1, T2, T3> &a0, T1 a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_of(
      std::move(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f120(const std::basic_string<T1, T2, T3> &a0,
     const std::basic_string<T1, T2, T3> &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_of(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f121(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_last_of(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a3));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f122(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_of(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f123(const std::basic_string<T1, T2, T3> &a0, T1 a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_of(
      std::move(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f124(const std::basic_string<T1, T2, T3> &a0,
     const std::basic_string<T1, T2, T3> &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_not_of(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f125(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_first_not_of(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a3));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f126(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_not_of(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f127(const std::basic_string<T1, T2, T3> &a0, T1 a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_not_of(
      std::move(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f128(const std::basic_string<T1, T2, T3> &a0,
     const std::basic_string<T1, T2, T3> &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_not_of(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f129(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_last_not_of(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a3));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f130(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_not_of(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f131(const std::basic_string<T1, T2, T3> &a0, T1 a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_not_of(
      std::move(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f132(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.substr(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3>
int f133(const std::basic_string<T1, T2, T3> &a0,
         const std::basic_string<T1, T2, T3> &a1) {
  return a0.compare(a1);
}

template <typename T1, typename T2, typename T3>
int f134(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2,
         const std::basic_string<T1, T2, T3> &a3) {
  return a0.compare(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2), a3);
}

template <typename T1, typename T2, typename T3>
int f135(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2,
         const std::basic_string<T1, T2, T3> &a3,
         typename std::basic_string<T1, T2, T3>::size_type a4,
         typename std::basic_string<T1, T2, T3>::size_type a5) {
  return a0.compare(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2), a3,
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a4),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a5));
}

template <typename T1, typename T2, typename T3>
int f136(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0.compare(a1);
}

template <typename T1, typename T2, typename T3>
int f137(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2, const T1 *a3) {
  return a0.compare(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2), a3);
}

template <typename T1, typename T2, typename T3>
int f138(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2, const T1 *a3,
         typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.compare(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2), a3,
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a4));
}

template <typename T1, typename T2, typename T3>
bool f139(const std::basic_string<T1, T2, T3> &a0,
          std::basic_string_view<T1, T2> a1) {
  return a0.starts_with(std::forward<std::basic_string_view<T1, T2>>(a1));
}

template <typename T1, typename T2, typename T3>
bool f140(const std::basic_string<T1, T2, T3> &a0, T1 a1) {
  return a0.starts_with(std::move(a1));
}

template <typename T1, typename T2, typename T3>
bool f141(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0.starts_with(a1);
}

template <typename T1, typename T2, typename T3>
bool f142(const std::basic_string<T1, T2, T3> &a0,
          std::basic_string_view<T1, T2> a1) {
  return a0.ends_with(std::forward<std::basic_string_view<T1, T2>>(a1));
}

template <typename T1, typename T2, typename T3>
bool f143(const std::basic_string<T1, T2, T3> &a0, T1 a1) {
  return a0.ends_with(std::move(a1));
}

template <typename T1, typename T2, typename T3>
bool f144(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0.ends_with(a1);
}

template <typename T1, typename T2, typename T3>
bool f145(const std::basic_string<T1, T2, T3> &a0,
          std::basic_string_view<T1, T2> a1) {
  return a0.contains(std::forward<std::basic_string_view<T1, T2>>(a1));
}

template <typename T1, typename T2, typename T3>
bool f146(const std::basic_string<T1, T2, T3> &a0, T1 a1) {
  return a0.contains(std::move(a1));
}

template <typename T1, typename T2, typename T3>
bool f147(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f148(const T1 *a0, const T3 &a1) {
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f149(typename std::basic_string<T1, T2, T3>::size_type a0, T1 a1,
     const T3 &a2) {
  return std::basic_string<T1, T2, T3>(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a0),
      std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> f150(std::from_range_t a0, T4 &&a1,
                                   const T3 &a2) {
  return std::basic_string<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T4>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
std::basic_string<T1, T2, T3> f151(T4 a0, T4 a1, const T3 &a2) {
  return std::basic_string<T1, T2, T3>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = std::enable_if_t<std::is_convertible_v<
              const T4 &, std::basic_string_view<T1, T2>>>>
std::basic_string<T1, T2, T3>
f152(const T4 &a0, typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T3 &a3)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return std::basic_string<T1, T2, T3>(
      a0, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2), a3);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename T5 = std::enable_if_t<
        std::integral_constant<
            bool,
            std::is_convertible<T4 const &,
                                std::basic_string_view<T1, T2>>::value &&
                std::integral_constant<
                    bool, !std::is_convertible<
                              T4 const *, const std::basic_string<T1, T2, T3>
                                              *>::value>::value &&
                std::integral_constant<
                    bool, !std::is_convertible<T4 const &, const T1 *>::value>::
                    value>::value,
        void>>
std::basic_string<T1, T2, T3> f153(const T4 &a0, const T3 &a1)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    std::basic_string<T1, T2, T3> &>
f154(std::basic_string<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template operator= <T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f155(std::basic_string<T1, T2, T3> &a0,
          typename std::basic_string<T1, T2, T3>::size_type a1, T4 a2) {
  return a0.template resize_and_overwrite<T4>(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    std::basic_string<T1, T2, T3> &>
f156(std::basic_string<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template operator+= <T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &f157(std::basic_string<T1, T2, T3> &a0,
                                    T4 &&a1) {
  return a0.template append_range<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T4>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
std::basic_string<T1, T2, T3> &f158(std::basic_string<T1, T2, T3> &a0, T4 a1,
                                    T4 a2) {
  return a0.template append<T4>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    std::basic_string<T1, T2, T3> &>
f159(std::basic_string<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template append<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    std::basic_string<T1, T2, T3> &>
f160(std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template append<T4>(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a3));
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T4>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
std::basic_string<T1, T2, T3> &f161(std::basic_string<T1, T2, T3> &a0, T4 a1,
                                    T4 a2) {
  return a0.template assign<T4>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &f162(std::basic_string<T1, T2, T3> &a0,
                                    T4 &&a1) {
  return a0.template assign_range<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    std::basic_string<T1, T2, T3> &>
f163(std::basic_string<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template assign<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    std::basic_string<T1, T2, T3> &>
f164(std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template assign<T4>(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a3));
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T4>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
typename std::basic_string<T1, T2, T3>::iterator
f165(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1, T4 a2, T4 a3) {
  return a0.template insert<T4>(
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a1),
      std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_string<T1, T2, T3>::iterator
f166(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1, T4 &&a2) {
  return a0.template insert_range<T4>(
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a1),
      std::forward<T4>(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    std::basic_string<T1, T2, T3> &>
f167(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, const T4 &a2)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template insert<T4>(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    std::basic_string<T1, T2, T3> &>
f168(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, const T4 &a2,
     typename std::basic_string<T1, T2, T3>::size_type a3,
     typename std::basic_string<T1, T2, T3>::size_type a4)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template insert<T4>(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1), a2,
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a3),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a4));
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T4>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
std::basic_string<T1, T2, T3> &
f169(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, T4 a3, T4 a4)
  requires(
      !(std::is_same<T4, T1 *>::value) &&
      !(std::is_same<T4, const T1 *>::value) &&
      !(std::is_same<
          T4, typename std::basic_string<T1, T2, T3>::iterator>::value) &&
      !(std::is_same<
          T4, typename std::basic_string<T1, T2, T3>::const_iterator>::value))
{
  return a0.template replace<T4>(
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a2),
      std::move(a3), std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &
f170(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, T4 &&a3) {
  return a0.template replace_with_range<T4>(
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a2),
      std::forward<T4>(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    std::basic_string<T1, T2, T3> &>
f171(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template replace<T4>(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2), a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    std::basic_string<T1, T2, T3> &>
f172(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3,
     typename std::basic_string<T1, T2, T3>::size_type a4,
     typename std::basic_string<T1, T2, T3>::size_type a5)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template replace<T4>(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2), a3,
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a4),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a5));
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    std::basic_string<T1, T2, T3> &>
f173(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, const T4 &a3)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template replace<T4>(
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::const_iterator>(a2),
      a3);
}

template <typename T1, typename T2, typename T3, typename T4>
typename __gnu_cxx::__alloc_traits<T3>::size_type
f174(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template find<T4>(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
typename __gnu_cxx::__alloc_traits<T3>::size_type
f175(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template rfind<T4>(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
typename __gnu_cxx::__alloc_traits<T3>::size_type
f176(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template find_first_of<T4>(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
typename __gnu_cxx::__alloc_traits<T3>::size_type
f177(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template find_last_of<T4>(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
typename __gnu_cxx::__alloc_traits<T3>::size_type
f178(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template find_first_not_of<T4>(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
typename __gnu_cxx::__alloc_traits<T3>::size_type
f179(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template find_last_not_of<T4>(
      a1, std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    int>
f180(const std::basic_string<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template compare<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    int>
f181(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template compare<T4>(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2), a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    int>
f182(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3,
     typename std::basic_string<T1, T2, T3>::size_type a4,
     typename std::basic_string<T1, T2, T3>::size_type a5)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template compare<T4>(
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a1),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a2), a3,
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a4),
      std::forward<typename std::basic_string<T1, T2, T3>::size_type>(a5));
}

template <typename T1, typename T2, typename T3>
using t15 = typename std::basic_string<T1, T2, T3>::iterator::iterator_type;

template <typename T1, typename T2, typename T3>
using t16 = typename std::basic_string<T1, T2, T3>::iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t17 = typename std::basic_string<T1, T2, T3>::iterator::value_type;

template <typename T1, typename T2, typename T3>
using t18 = typename std::basic_string<T1, T2, T3>::iterator::difference_type;

template <typename T1, typename T2, typename T3>
using t19 = typename std::basic_string<T1, T2, T3>::iterator::reference;

template <typename T1, typename T2, typename T3>
using t20 = typename std::basic_string<T1, T2, T3>::iterator::pointer;

template <typename T1, typename T2, typename T3>
using t21 = typename std::basic_string<T1, T2, T3>::iterator::iterator_concept;

template <typename T1, typename T2, typename T3>
using t22 =
    typename std::basic_string<T1, T2, T3>::const_iterator::iterator_type;

template <typename T1, typename T2, typename T3>
using t23 =
    typename std::basic_string<T1, T2, T3>::const_iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t24 = typename std::basic_string<T1, T2, T3>::const_iterator::value_type;

template <typename T1, typename T2, typename T3>
using t25 =
    typename std::basic_string<T1, T2, T3>::const_iterator::difference_type;

template <typename T1, typename T2, typename T3>
using t26 = typename std::basic_string<T1, T2, T3>::const_iterator::reference;

template <typename T1, typename T2, typename T3>
using t27 = typename std::basic_string<T1, T2, T3>::const_iterator::pointer;

template <typename T1, typename T2, typename T3>
using t28 =
    typename std::basic_string<T1, T2, T3>::const_iterator::iterator_concept;

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type f183() {
  return std::basic_string<T1, T2, T3>::npos;
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f184(std::basic_istream<T1, T2> &&a0,
                                 std::basic_string<T1, T2, T3> &a1) {
  return std::getline(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f185(std::basic_istream<T1, T2> &&a0,
                                 std::basic_string<T1, T2, T3> &a1, T1 a2) {
  return std::getline(std::move(a0), a1, std::move(a2));
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f186(std::basic_istream<T1, T2> &a0,
                                 std::basic_string<T1, T2, T3> &a1) {
  return std::getline(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f187(std::basic_istream<T1, T2> &a0,
                                 std::basic_string<T1, T2, T3> &a1, T1 a2) {
  return std::getline(a0, a1, std::move(a2));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f188(std::basic_string<T1, T2, T3> &&a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return std::operator+(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f189(std::basic_string<T1, T2, T3> &&a0,
                                   const T1 *a1) {
  return std::operator+(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f190(std::basic_string<T1, T2, T3> &&a0,
                                   std::basic_string<T1, T2, T3> &&a1) {
  return std::operator+(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f191(std::basic_string<T1, T2, T3> &&a0, T1 a1) {
  return std::operator+(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f192(const std::basic_string<T1, T2, T3> &a0,
                                   std::basic_string<T1, T2, T3> &&a1) {
  return std::operator+(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f193(const std::basic_string<T1, T2, T3> &a0,
                                   const T1 *a1) {
  return std::operator+(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f194(const std::basic_string<T1, T2, T3> &a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return std::operator+(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f195(const std::basic_string<T1, T2, T3> &a0,
                                   T1 a1) {
  return std::operator+(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f196(const T1 *a0,
                                   std::basic_string<T1, T2, T3> &&a1) {
  return std::operator+(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f197(const T1 *a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return std::operator+(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f198(T1 a0, std::basic_string<T1, T2, T3> &&a1) {
  return std::operator+(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f199(T1 a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return std::operator+(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f200(std::basic_ostream<T1, T2> &a0,
                                 const std::basic_string<T1, T2, T3> &a1) {
  return std::operator<<(a0, a1);
}

template <typename T1, typename T2, typename T3>
auto f201(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return std::operator<=>(a0, a1);
}

template <typename T1, typename T2, typename T3>
auto f202(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return std::operator<=>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f203(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return std::operator==(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f204(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return std::operator==(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f205(std::basic_istream<T1, T2> &a0,
                                 std::basic_string<T1, T2, T3> &a1) {
  return std::operator>>(a0, a1);
}

double f206(const std::string &a0, std::size_t *a1) {
  return std::stod(a0, a1);
}

double f207(const std::wstring &a0, std::size_t *a1) {
  return std::stod(a0, a1);
}

float f208(const std::string &a0, std::size_t *a1) { return std::stof(a0, a1); }

float f209(const std::wstring &a0, std::size_t *a1) {
  return std::stof(a0, a1);
}

int f210(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoi(a0, a1, a2);
}

int f211(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoi(a0, a1, a2);
}

long f212(const std::string &a0, std::size_t *a1, int a2) {
  return std::stol(a0, a1, a2);
}

long f213(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stol(a0, a1, a2);
}

long double f214(const std::string &a0, std::size_t *a1) {
  return std::stold(a0, a1);
}

long double f215(const std::wstring &a0, std::size_t *a1) {
  return std::stold(a0, a1);
}

long long f216(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoll(a0, a1, a2);
}

long long f217(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoll(a0, a1, a2);
}

unsigned long f218(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoul(a0, a1, a2);
}

unsigned long f219(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoul(a0, a1, a2);
}

unsigned long long f220(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoull(a0, a1, a2);
}

unsigned long long f221(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoull(a0, a1, a2);
}

std::basic_string<char> f222(const char *a0, std::size_t a1) {
  return std::operator""s(a0, a1);
}

std::basic_string<wchar_t> f223(const wchar_t *a0, std::size_t a1) {
  return std::operator""s(a0, a1);
}

std::basic_string<char16_t> f224(const char16_t *a0, std::size_t a1) {
  return std::operator""s(a0, a1);
}

std::basic_string<char8_t> f225(const char8_t *a0, std::size_t a1) {
  return std::operator""s(a0, a1);
}

std::basic_string<char32_t> f226(const char32_t *a0, std::size_t a1) {
  return std::operator""s(a0, a1);
}

template <typename T1, typename T2, typename T3>
void f227(std::basic_string<T1, T2, T3> &a0,
          std::basic_string<T1, T2, T3> &a1) {
  return std::swap(a0, a1);
}

std::wstring f228(long double a0) { return std::to_wstring(a0); }

std::wstring f229(int a0) { return std::to_wstring(a0); }

std::wstring f230(long long a0) { return std::to_wstring(a0); }

std::wstring f231(long a0) { return std::to_wstring(a0); }

std::wstring f232(double a0) { return std::to_wstring(a0); }

std::wstring f233(float a0) { return std::to_wstring(a0); }

std::wstring f234(unsigned int a0) { return std::to_wstring(a0); }

std::wstring f235(unsigned long long a0) { return std::to_wstring(a0); }

std::wstring f236(unsigned long a0) { return std::to_wstring(a0); }
