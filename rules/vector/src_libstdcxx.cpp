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

template <typename T1> std::vector<bool, T1> f85(std::size_t a0, const T1 &a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <typename T1>
std::vector<bool, T1> f86(std::size_t a0, const bool &a1, const T1 &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <typename T1>
void f87(std::vector<bool, T1> &a0, std::size_t a1, const bool &a2) {
  return a0.assign(a1, a2);
}

template <typename T1> std::size_t f88(const std::vector<bool, T1> &a0) {
  return a0.size();
}

template <typename T1> std::size_t f89(const std::vector<bool, T1> &a0) {
  return a0.max_size();
}

template <typename T1> std::size_t f90(const std::vector<bool, T1> &a0) {
  return a0.capacity();
}

template <typename T1>
typename std::vector<bool, T1>::reference f91(std::vector<bool, T1> &a0,
                                              std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1>
bool f92(const std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1>
typename std::vector<bool, T1>::reference f93(std::vector<bool, T1> &a0,
                                              std::size_t a1) {
  return a0.at(a1);
}

template <typename T1>
bool f94(const std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.at(a1);
}

template <typename T1> void f95(std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.reserve(a1);
}

template <typename T1> bool f96(const std::vector<bool, T1> &a0) {
  return a0.front();
}

template <typename T1> bool f97(const std::vector<bool, T1> &a0) {
  return a0.back();
}

template <typename T1> void f98(std::vector<bool, T1> &a0, bool a1) {
  return a0.push_back(a1);
}

template <typename T1>
void f99(typename std::vector<bool, T1>::reference a0,
         typename std::vector<bool, T1>::reference a1) {
  return std::vector<bool, T1>::swap(a0, a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f100(std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::const_iterator a1) {
  return a0.insert(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f101(std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::const_iterator a1, std::size_t a2,
     const bool &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1>
void f102(std::vector<bool, T1> &a0, std::size_t a1, bool a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2, typename T3 = std::_RequireInputIter<T2>>
std::vector<bool, T1> f103(T2 a0, T2 a1, const T1 &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3 = std::_RequireInputIter<T2>>
void f104(std::vector<bool, T1> &a0, T2 a1, T2 a2) {
  return a0.template assign<T2>(a1, a2);
}

template <typename T1, typename T2, typename T3 = std::_RequireInputIter<T2>>
typename std::vector<bool, T1>::iterator
f105(std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::const_iterator a1, T2 a2, T2 a3) {
  return a0.template insert<T2>(a1, a2, a3);
}

template <typename T1, typename... T2>
typename std::vector<bool, T1>::iterator
f106(std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::const_iterator a1, T2 &&...a2) {
  return a0.template emplace<T2...>(a1, std::forward<T2>(a2)...);
}

template <typename T1>
typename std::vector<bool, T1>::reference
f107(const typename std::vector<bool, T1>::reference &a0) {
  return typename std::vector<bool, T1>::reference(a0);
}

template <typename T1>
bool f108(const typename std::vector<bool, T1>::reference &a0) {
  return static_cast<bool>(a0);
}

template <typename T1>
typename std::vector<bool, T1>::reference &
f109(typename std::vector<bool, T1>::reference &a0, bool a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reference &
f110(typename std::vector<bool, T1>::reference &a0,
     const typename std::vector<bool, T1>::reference &a1) {
  return a0.operator=(a1);
}

template <typename T1>
bool f111(const typename std::vector<bool, T1>::reference &a0,
          const typename std::vector<bool, T1>::reference &a1) {
  return a0.operator==(a1);
}

template <typename T1>
bool f112(const typename std::vector<bool, T1>::reference &a0,
          const typename std::vector<bool, T1>::reference &a1) {
  return a0.operator<(a1);
}

template <typename T1>
void f113(typename std::vector<bool, T1>::reference &a0) {
  return a0.flip();
}

template <typename T1>
void f114(bool &a0, typename std::vector<bool, T1>::reference a1) {
  return swap(a0, a1);
}

template <typename T1>
void f115(typename std::vector<bool, T1>::reference a0, bool &a1) {
  return swap(a0, a1);
}

template <typename T1>
void f116(typename std::vector<bool, T1>::reference a0,
          typename std::vector<bool, T1>::reference a1) {
  return swap(a0, a1);
}

template <typename T1>
using t35 = typename std::vector<bool, T1>::iterator::reference;

template <typename T1>
using t36 = typename std::vector<bool, T1>::iterator::pointer;

template <typename T1>
using t37 = typename std::vector<bool, T1>::iterator::iterator;

template <typename T1> typename std::vector<bool, T1>::iterator f117() {
  return typename std::vector<bool, T1>::iterator();
}

template <typename T1>
typename std::vector<bool, T1>::iterator f118(unsigned long *a0,
                                              unsigned int a1) {
  return typename std::vector<bool, T1>::iterator(a0, a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f119(typename std::vector<bool, T1>::iterator &a0) {
  return a0.operator++();
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f120(typename std::vector<bool, T1>::iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f121(typename std::vector<bool, T1>::iterator &a0) {
  return a0.operator--();
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f122(typename std::vector<bool, T1>::iterator &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f123(typename std::vector<bool, T1>::iterator &a0, long a1) {
  return a0.operator+=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f124(typename std::vector<bool, T1>::iterator &a0, long a1) {
  return a0.operator-=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f125(const typename std::vector<bool, T1>::iterator &a0) {
  return typename std::vector<bool, T1>::iterator(a0);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f126(typename std::vector<bool, T1>::iterator &&a0) {
  return typename std::vector<bool, T1>::iterator(std::move(a0));
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f127(typename std::vector<bool, T1>::iterator &a0,
     const typename std::vector<bool, T1>::iterator &a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f128(typename std::vector<bool, T1>::iterator &a0,
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

template <typename T1> typename std::vector<bool, T1>::const_iterator f129() {
  return typename std::vector<bool, T1>::const_iterator();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator f130(unsigned long *a0,
                                                    unsigned int a1) {
  return typename std::vector<bool, T1>::const_iterator(a0, a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f131(typename std::vector<bool, T1>::const_iterator &a0) {
  return a0.operator++();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f132(typename std::vector<bool, T1>::const_iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f133(typename std::vector<bool, T1>::const_iterator &a0) {
  return a0.operator--();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f134(typename std::vector<bool, T1>::const_iterator &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f135(typename std::vector<bool, T1>::const_iterator &a0, long a1) {
  return a0.operator+=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f136(typename std::vector<bool, T1>::const_iterator &a0, long a1) {
  return a0.operator-=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f137(const typename std::vector<bool, T1>::const_iterator &a0) {
  return typename std::vector<bool, T1>::const_iterator(a0);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f138(typename std::vector<bool, T1>::const_iterator &&a0) {
  return typename std::vector<bool, T1>::const_iterator(std::move(a0));
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f139(typename std::vector<bool, T1>::const_iterator &a0,
     const typename std::vector<bool, T1>::const_iterator &a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f140(typename std::vector<bool, T1>::const_iterator &a0,
     typename std::vector<bool, T1>::const_iterator &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::vector<T1, T2> f141(std::size_t a0, const T2 &a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> f142(std::size_t a0, const T1 &a1, const T2 &a2) {
  return std::vector<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
void f143(std::vector<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
std::size_t f144(const std::vector<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
std::size_t f145(const std::vector<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
void f146(std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f147(std::vector<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2>
std::size_t f148(const std::vector<T1, T2> &a0) {
  return a0.capacity();
}

template <typename T1, typename T2>
void f149(std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.reserve(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f150(std::vector<T1, T2> &a0,
                                             std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f151(const std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f152(std::vector<T1, T2> &a0,
                                             std::size_t a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f153(const std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f154(std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f155(const std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f156(std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f157(const std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f158(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     std::size_t a2, const T1 &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
std::vector<T1, T2> f159(T3 a0, T3 a1, const T2 &a2) {
  return std::vector<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
void f160(std::vector<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
typename std::vector<T1, T2>::iterator
f161(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     T3 a2, T3 a3) {
  return a0.template insert<T3>(a1, a2, a3);
}

template <typename T1, typename T2>
using t42 = typename std::vector<T1, T2>::iterator::iterator_type;

template <typename T1, typename T2>
using t43 = typename std::vector<T1, T2>::const_iterator::iterator_type;
