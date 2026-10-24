#include <algorithm>
#include <utility>
#include <vector>

template <typename T1> std::vector<bool, T1> f58(std::size_t a0, const T1 &a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <typename T1>
std::vector<bool, T1> f59(std::size_t a0, const bool &a1, const T1 &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <typename T1>
void f60(std::vector<bool, T1> &a0, std::size_t a1, const bool &a2) {
  return a0.assign(a1, a2);
}

template <typename T1> std::_Bit_iterator f61(std::vector<bool, T1> &a0) {
  return a0.begin();
}

template <typename T1>
std::_Bit_const_iterator f62(const std::vector<bool, T1> &a0) {
  return a0.begin();
}

template <typename T1> std::_Bit_iterator f63(std::vector<bool, T1> &a0) {
  return a0.end();
}

template <typename T1>
std::_Bit_const_iterator f64(const std::vector<bool, T1> &a0) {
  return a0.end();
}

template <typename T1>
std::reverse_iterator<std::_Bit_iterator> f65(std::vector<bool, T1> &a0) {
  return a0.rbegin();
}

template <typename T1>
std::reverse_iterator<std::_Bit_const_iterator>
f66(const std::vector<bool, T1> &a0) {
  return a0.rbegin();
}

template <typename T1>
std::reverse_iterator<std::_Bit_iterator> f67(std::vector<bool, T1> &a0) {
  return a0.rend();
}

template <typename T1>
std::reverse_iterator<std::_Bit_const_iterator>
f68(const std::vector<bool, T1> &a0) {
  return a0.rend();
}

template <typename T1>
std::_Bit_const_iterator f69(const std::vector<bool, T1> &a0) {
  return a0.cbegin();
}

template <typename T1>
std::_Bit_const_iterator f70(const std::vector<bool, T1> &a0) {
  return a0.cend();
}

template <typename T1>
std::reverse_iterator<std::_Bit_const_iterator>
f71(const std::vector<bool, T1> &a0) {
  return a0.crbegin();
}

template <typename T1>
std::reverse_iterator<std::_Bit_const_iterator>
f72(const std::vector<bool, T1> &a0) {
  return a0.crend();
}

template <typename T1> std::size_t f73(const std::vector<bool, T1> &a0) {
  return a0.size();
}

template <typename T1> std::size_t f74(const std::vector<bool, T1> &a0) {
  return a0.max_size();
}

template <typename T1> std::size_t f75(const std::vector<bool, T1> &a0) {
  return a0.capacity();
}

template <typename T1>
std::_Bit_reference f76(std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1>
bool f77(const std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::_Bit_reference f78(std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.at(a1);
}

template <typename T1>
bool f79(const std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.at(a1);
}

template <typename T1> void f80(std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.reserve(a1);
}

template <typename T1> std::_Bit_reference f81(std::vector<bool, T1> &a0) {
  return a0.front();
}

template <typename T1> bool f82(const std::vector<bool, T1> &a0) {
  return a0.front();
}

template <typename T1> std::_Bit_reference f83(std::vector<bool, T1> &a0) {
  return a0.back();
}

template <typename T1> bool f84(const std::vector<bool, T1> &a0) {
  return a0.back();
}

template <typename T1> void f85(std::vector<bool, T1> &a0, bool a1) {
  return a0.push_back(a1);
}

template <typename T1>
void f86(std::_Bit_reference a0, std::_Bit_reference a1) {
  return std::vector<bool, T1>::swap(std::move(a0), std::move(a1));
}

template <typename T1>
std::_Bit_iterator f87(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1,
                       const bool &a2) {
  return a0.insert(std::move(a1), a2);
}

template <typename T1>
std::_Bit_iterator f88(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1) {
  return a0.insert(std::move(a1));
}

template <typename T1>
std::_Bit_iterator f89(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1,
                       std::size_t a2, const bool &a3) {
  return a0.insert(std::move(a1), a2, a3);
}

template <typename T1>
std::_Bit_iterator f90(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1,
                       std::initializer_list<bool> a2) {
  return a0.insert(std::move(a1), std::move(a2));
}

template <typename T1>
std::_Bit_iterator f91(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1) {
  return a0.erase(std::move(a1));
}

template <typename T1>
std::_Bit_iterator f92(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1,
                       std::_Bit_const_iterator a2) {
  return a0.erase(std::move(a1), std::move(a2));
}

template <typename T1>
void f93(std::vector<bool, T1> &a0, std::size_t a1, bool a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2, typename T3 = std::_RequireInputIter<T2>>
std::vector<bool, T1> f94(T2 a0, T2 a1, const T1 &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3 = std::_RequireInputIter<T2>>
void f95(std::vector<bool, T1> &a0, T2 a1, T2 a2) {
  return a0.template assign<T2>(a1, a2);
}

template <typename T1, typename T2, typename T3 = std::_RequireInputIter<T2>>
std::_Bit_iterator f96(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1,
                       T2 a2, T2 a3) {
  return a0.template insert<T2>(std::move(a1), a2, a3);
}

template <typename T1, typename... T2>
std::_Bit_iterator f97(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1,
                       T2 &&...a2) {
  return a0.template emplace<T2...>(std::move(a1), std::forward<T2>(a2)...);
}

template <typename T1, typename T2>
std::vector<T1, T2> f98(std::size_t a0, const T2 &a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> f99(std::size_t a0, const T1 &a1, const T2 &a2) {
  return std::vector<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
void f100(std::vector<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
std::size_t f101(const std::vector<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
std::size_t f102(const std::vector<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
void f103(std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f104(std::vector<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2>
std::size_t f105(const std::vector<T1, T2> &a0) {
  return a0.capacity();
}

template <typename T1, typename T2>
void f106(std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.reserve(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f107(std::vector<T1, T2> &a0,
                                             std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f108(const std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f109(std::vector<T1, T2> &a0,
                                             std::size_t a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f110(const std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f111(std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f112(const std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f113(std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f114(const std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f115(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     std::size_t a2, const T1 &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
std::vector<T1, T2> f116(T3 a0, T3 a1, const T2 &a2) {
  return std::vector<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
void f117(std::vector<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
typename std::vector<T1, T2>::iterator
f118(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     T3 a2, T3 a3) {
  return a0.template insert<T3>(a1, a2, a3);
}

template <typename T1, typename T2>
using t39 = typename std::vector<T1, T2>::iterator::iterator_type;

template <typename T1, typename T2>
using t40 = typename std::vector<T1, T2>::const_iterator::iterator_type;
