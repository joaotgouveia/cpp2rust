#include <initializer_list>
#include <list>
#include <utility>

template <typename T1, typename T2>
bool f92(const std::list<T1, T2> &a0, const std::list<T1, T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
void f93(std::list<T1, T2> &a0, std::list<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::list<T1, T2> f94(typename std::list<T1, T2>::size_type a0) {
  return std::list<T1, T2>(a0);
}

template <typename T1, typename T2>
std::list<T1, T2> f95(typename std::list<T1, T2>::size_type a0, const T2 &a1) {
  return std::list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::list<T1, T2> f96(typename std::list<T1, T2>::size_type a0, const T1 &a1) {
  return std::list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::list<T1, T2> f97(std::initializer_list<T1> a0) {
  return std::list<T1, T2>(a0);
}

template <typename T1, typename T2>
void f98(std::list<T1, T2> &a0, typename std::list<T1, T2>::size_type a1,
         const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
typename std::list<T1, T2>::size_type f99(const std::list<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::size_type f100(const std::list<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2> T1 &f101(std::list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
const T1 &f102(const std::list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2> T1 &f103(std::list<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
const T1 &f104(const std::list<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator
f105(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
     typename std::list<T1, T2>::size_type a2, const T1 &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2>
void f106(std::list<T1, T2> &a0, typename std::list<T1, T2>::size_type a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f107(std::list<T1, T2> &a0, typename std::list<T1, T2>::size_type a1,
          const T1 &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2>
using t11 = typename std::list<T1, T2>::iterator::iterator_category;

template <typename T1, typename T2>
using t12 = typename std::list<T1, T2>::iterator::value_type;

template <typename T1, typename T2>
using t13 = typename std::list<T1, T2>::iterator::reference;

template <typename T1, typename T2>
using t14 = typename std::list<T1, T2>::iterator::pointer;

template <typename T1, typename T2>
using t15 = typename std::list<T1, T2>::iterator::difference_type;

template <typename T1, typename T2>
using t16 = typename std::list<T1, T2>::const_iterator::iterator_category;

template <typename T1, typename T2>
using t17 = typename std::list<T1, T2>::const_iterator::value_type;

template <typename T1, typename T2>
using t18 = typename std::list<T1, T2>::const_iterator::reference;

template <typename T1, typename T2>
using t19 = typename std::list<T1, T2>::const_iterator::pointer;

template <typename T1, typename T2>
using t20 = typename std::list<T1, T2>::const_iterator::difference_type;
