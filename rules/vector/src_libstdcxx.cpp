#include <algorithm>
#include <utility>
#include <vector>

template <typename T1> using t27 = typename std::vector<bool, T1>::reference;

template <typename T1>
using t28 = typename std::vector<bool, T1>::const_reference;

template <typename T1> using t29 = typename std::vector<bool, T1>::pointer;

template <typename T1>
using t30 = typename std::vector<bool, T1>::const_pointer;

template <typename T1> using t31 = typename std::vector<bool, T1>::iterator;

template <typename T1>
using t32 = typename std::vector<bool, T1>::const_iterator;

template <typename T1>
using t33 = typename std::vector<bool, T1>::const_reverse_iterator;

template <typename T1>
using t34 = typename std::vector<bool, T1>::reverse_iterator;

template <typename T1> std::vector<bool, T1> f77(std::size_t a0, const T1 &a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <typename T1>
std::vector<bool, T1> f78(std::size_t a0, const bool &a1, const T1 &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <typename T1>
void f79(std::vector<bool, T1> &a0, std::size_t a1, const bool &a2) {
  return a0.assign(a1, a2);
}

template <typename T1> std::size_t f80(const std::vector<bool, T1> &a0) {
  return a0.size();
}

template <typename T1> std::size_t f81(const std::vector<bool, T1> &a0) {
  return a0.max_size();
}

template <typename T1> std::size_t f82(const std::vector<bool, T1> &a0) {
  return a0.capacity();
}

template <typename T1>
typename std::vector<bool, T1>::reference f83(std::vector<bool, T1> &a0,
                                              std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1>
bool f84(const std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1>
typename std::vector<bool, T1>::reference f85(std::vector<bool, T1> &a0,
                                              std::size_t a1) {
  return a0.at(a1);
}

template <typename T1>
bool f86(const std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.at(a1);
}

template <typename T1> void f87(std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.reserve(a1);
}

template <typename T1> bool f88(const std::vector<bool, T1> &a0) {
  return a0.front();
}

template <typename T1> bool f89(const std::vector<bool, T1> &a0) {
  return a0.back();
}

template <typename T1> void f90(std::vector<bool, T1> &a0, bool a1) {
  return a0.push_back(a1);
}

template <typename T1>
void f91(typename std::vector<bool, T1>::reference a0,
         typename std::vector<bool, T1>::reference a1) {
  return std::vector<bool, T1>::swap(a0, a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f92(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1) {
  return a0.insert(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f93(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1, std::size_t a2,
    const bool &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1>
void f94(std::vector<bool, T1> &a0, std::size_t a1, bool a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2, typename T3 = std::_RequireInputIter<T2>>
std::vector<bool, T1> f95(T2 a0, T2 a1, const T1 &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3 = std::_RequireInputIter<T2>>
void f96(std::vector<bool, T1> &a0, T2 a1, T2 a2) {
  return a0.template assign<T2>(a1, a2);
}

template <typename T1, typename T2, typename T3 = std::_RequireInputIter<T2>>
typename std::vector<bool, T1>::iterator
f97(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1, T2 a2, T2 a3) {
  return a0.template insert<T2>(a1, a2, a3);
}

template <typename T1, typename... T2>
typename std::vector<bool, T1>::iterator
f98(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1, T2 &&...a2) {
  return a0.template emplace<T2...>(a1, std::forward<T2>(a2)...);
}

template <typename T1>
typename std::vector<bool, T1>::reference
f99(const typename std::vector<bool, T1>::reference &a0) {
  return typename std::vector<bool, T1>::reference(a0);
}

template <typename T1>
bool f100(const typename std::vector<bool, T1>::reference &a0) {
  return static_cast<bool>(a0);
}

template <typename T1>
typename std::vector<bool, T1>::reference &
f101(typename std::vector<bool, T1>::reference &a0, bool a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reference &
f102(typename std::vector<bool, T1>::reference &a0,
     const typename std::vector<bool, T1>::reference &a1) {
  return a0.operator=(a1);
}

template <typename T1>
bool f103(const typename std::vector<bool, T1>::reference &a0,
          const typename std::vector<bool, T1>::reference &a1) {
  return a0.operator==(a1);
}

template <typename T1>
bool f104(const typename std::vector<bool, T1>::reference &a0,
          const typename std::vector<bool, T1>::reference &a1) {
  return a0.operator<(a1);
}

template <typename T1>
void f105(typename std::vector<bool, T1>::reference &a0) {
  return a0.flip();
}

template <typename T1>
void f106(bool &a0, typename std::vector<bool, T1>::reference a1) {
  return swap(a0, a1);
}

template <typename T1>
void f107(typename std::vector<bool, T1>::reference a0, bool &a1) {
  return swap(a0, a1);
}

template <typename T1>
void f108(typename std::vector<bool, T1>::reference a0,
          typename std::vector<bool, T1>::reference a1) {
  return swap(a0, a1);
}

template <typename T1>
using t35 = typename std::vector<bool, T1>::iterator::reference;

template <typename T1>
using t36 = typename std::vector<bool, T1>::iterator::pointer;

template <typename T1>
using t37 = typename std::vector<bool, T1>::iterator::iterator;

template <typename T1> typename std::vector<bool, T1>::iterator f109() {
  return typename std::vector<bool, T1>::iterator();
}

template <typename T1>
typename std::vector<bool, T1>::iterator f110(unsigned long *a0,
                                              unsigned int a1) {
  return typename std::vector<bool, T1>::iterator(a0, a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f111(typename std::vector<bool, T1>::iterator &a0) {
  return a0.operator++();
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f112(typename std::vector<bool, T1>::iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f113(typename std::vector<bool, T1>::iterator &a0) {
  return a0.operator--();
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f114(typename std::vector<bool, T1>::iterator &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f115(typename std::vector<bool, T1>::iterator &a0, long a1) {
  return a0.operator+=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f116(typename std::vector<bool, T1>::iterator &a0, long a1) {
  return a0.operator-=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f117(const typename std::vector<bool, T1>::iterator &a0) {
  return typename std::vector<bool, T1>::iterator(a0);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f118(typename std::vector<bool, T1>::iterator &&a0) {
  return typename std::vector<bool, T1>::iterator(std::move(a0));
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f119(typename std::vector<bool, T1>::iterator &a0,
     const typename std::vector<bool, T1>::iterator &a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f120(typename std::vector<bool, T1>::iterator &a0,
     typename std::vector<bool, T1>::iterator &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
using t38 = typename std::vector<bool, T1>::const_iterator::reference;

template <typename T1>
using t39 = typename std::vector<bool, T1>::const_iterator::const_reference;

template <typename T1>
using t40 = typename std::vector<bool, T1>::const_iterator::pointer;

template <typename T1>
using t41 = typename std::vector<bool, T1>::const_iterator::const_iterator;

template <typename T1> typename std::vector<bool, T1>::const_iterator f121() {
  return typename std::vector<bool, T1>::const_iterator();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator f122(unsigned long *a0,
                                                    unsigned int a1) {
  return typename std::vector<bool, T1>::const_iterator(a0, a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f123(typename std::vector<bool, T1>::const_iterator &a0) {
  return a0.operator++();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f124(typename std::vector<bool, T1>::const_iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f125(typename std::vector<bool, T1>::const_iterator &a0) {
  return a0.operator--();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f126(typename std::vector<bool, T1>::const_iterator &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f127(typename std::vector<bool, T1>::const_iterator &a0, long a1) {
  return a0.operator+=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f128(typename std::vector<bool, T1>::const_iterator &a0, long a1) {
  return a0.operator-=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f129(const typename std::vector<bool, T1>::const_iterator &a0) {
  return typename std::vector<bool, T1>::const_iterator(a0);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f130(typename std::vector<bool, T1>::const_iterator &&a0) {
  return typename std::vector<bool, T1>::const_iterator(std::move(a0));
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f131(typename std::vector<bool, T1>::const_iterator &a0,
     const typename std::vector<bool, T1>::const_iterator &a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f132(typename std::vector<bool, T1>::const_iterator &a0,
     typename std::vector<bool, T1>::const_iterator &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::vector<T1, T2> f133(std::size_t a0, const T2 &a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> f134(std::size_t a0, const T1 &a1, const T2 &a2) {
  return std::vector<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
void f135(std::vector<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
std::size_t f136(const std::vector<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
std::size_t f137(const std::vector<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
void f138(std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f139(std::vector<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2>
std::size_t f140(const std::vector<T1, T2> &a0) {
  return a0.capacity();
}

template <typename T1, typename T2>
void f141(std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.reserve(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f142(std::vector<T1, T2> &a0,
                                             std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f143(const std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f144(std::vector<T1, T2> &a0,
                                             std::size_t a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f145(const std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f146(std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f147(const std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f148(std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f149(const std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f150(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     std::size_t a2, const T1 &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
std::vector<T1, T2> f151(T3 a0, T3 a1, const T2 &a2) {
  return std::vector<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
void f152(std::vector<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
typename std::vector<T1, T2>::iterator
f153(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     T3 a2, T3 a3) {
  return a0.template insert<T3>(a1, a2, a3);
}

template <typename T1, typename T2>
using t42 = typename std::vector<T1, T2>::iterator::iterator_type;

template <typename T1, typename T2>
using t43 = typename std::vector<T1, T2>::const_iterator::iterator_type;
