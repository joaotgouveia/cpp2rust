#include <initializer_list>
#include <utility>
#include <vector>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
std::vector<bool, T1> f198(typename std::vector<bool, T1>::size_type a0) {
  return std::vector<bool, T1>(a0);
}

template <typename T1>
std::vector<bool, T1>
f199(typename std::vector<bool, T1>::size_type a0,
     const typename std::vector<bool, T1>::value_type &a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <typename T1>
std::vector<bool, T1>
f200(typename std::vector<bool, T1>::size_type a0,
     const typename std::vector<bool, T1>::value_type &a1,
     const typename std::vector<bool, T1>::allocator_type &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <typename T1>
std::vector<bool, T1>
f201(std::initializer_list<typename std::vector<bool, T1>::value_type> a0) {
  return std::vector<bool, T1>(a0);
}

template <typename T1>
std::vector<bool, T1>
f202(std::initializer_list<typename std::vector<bool, T1>::value_type> a0,
     const typename std::vector<bool, T1>::allocator_type &a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <typename T1>
std::vector<bool, T1> &
f203(std::vector<bool, T1> &a0,
     std::initializer_list<typename std::vector<bool, T1>::value_type> a1) {
  return a0.operator=(a1);
}

template <typename T1>
void f204(std::vector<bool, T1> &a0,
          typename std::vector<bool, T1>::size_type a1,
          const typename std::vector<bool, T1>::value_type &a2) {
  return a0.assign(a1, a2);
}

template <typename T1>
void f205(
    std::vector<bool, T1> &a0,
    std::initializer_list<typename std::vector<bool, T1>::value_type> a1) {
  return a0.assign(a1);
}

template <typename T1>
void f206(std::vector<bool, T1> &a0,
          const typename std::vector<bool, T1>::value_type &a1) {
  return a0.push_back(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f207(std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::const_iterator a1,
     const typename std::vector<bool, T1>::value_type &a2) {
  return a0.insert(a1, a2);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f208(std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::const_iterator a1,
     typename std::vector<bool, T1>::size_type a2,
     const typename std::vector<bool, T1>::value_type &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f209(std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::const_iterator a1,
     std::initializer_list<typename std::vector<bool, T1>::value_type> a2) {
  return a0.insert(a1, a2);
}

template <typename T1>
void f210(std::vector<bool, T1> &a0,
          typename std::vector<bool, T1>::size_type a1,
          typename std::vector<bool, T1>::value_type a2) {
  return a0.resize(a1, a2);
}

template <typename T1>
using t69 = typename std::vector<bool, T1>::pointer::difference_type;

template <typename T1>
using t70 = typename std::vector<bool, T1>::pointer::value_type;

template <typename T1>
using t71 = typename std::vector<bool, T1>::pointer::iterator_category;

template <typename T1>
using t72 = typename std::vector<bool, T1>::const_pointer::difference_type;

template <typename T1>
using t73 = typename std::vector<bool, T1>::const_pointer::value_type;

template <typename T1>
using t74 = typename std::vector<bool, T1>::const_pointer::iterator_category;

template <typename T1>
using t75 = typename std::vector<bool, T1>::iterator::difference_type;

template <typename T1>
using t76 = typename std::vector<bool, T1>::iterator::value_type;

template <typename T1>
using t77 = typename std::vector<bool, T1>::iterator::iterator_category;

template <typename T1>
using t78 = typename std::vector<bool, T1>::const_iterator::difference_type;

template <typename T1>
using t79 = typename std::vector<bool, T1>::const_iterator::value_type;

template <typename T1>
using t80 = typename std::vector<bool, T1>::const_iterator::iterator_category;

template <typename T1>
using t81 = typename std::vector<bool, T1>::reverse_iterator::iterator_category;

template <typename T1>
using t82 = typename std::vector<bool, T1>::reverse_iterator::value_type;

template <typename T1>
using t83 =
    typename std::vector<bool, T1>::const_reverse_iterator::iterator_category;

template <typename T1>
using t84 = typename std::vector<bool, T1>::const_reverse_iterator::value_type;

template <typename T1, typename T2>
std::vector<T1, T2> f211(typename std::vector<T1, T2>::size_type a0) {
  return std::vector<T1, T2>(a0);
}

template <typename T1, typename T2>
std::vector<T1, T2> f212(typename std::vector<T1, T2>::size_type a0,
                         const typename std::vector<T1, T2>::value_type &a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::vector<T1, T2>
f213(std::initializer_list<typename std::vector<T1, T2>::value_type> a0) {
  return std::vector<T1, T2>(a0);
}

template <typename T1, typename T2>
void f214(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1,
          const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::allocator_type
f215(const std::vector<T1, T2> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2>
T1 &f216(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
const T1 &f217(const std::vector<T1, T2> &a0,
               typename std::vector<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
T1 &f218(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
const T1 &f219(const std::vector<T1, T2> &a0,
               typename std::vector<T1, T2>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2> T1 &f220(std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
const T1 &f221(const std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2> T1 &f222(std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
const T1 &f223(const std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::value_type *f224(std::vector<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
const typename std::vector<T1, T2>::value_type *
f225(const std::vector<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
void f226(std::vector<T1, T2> &a0, const T1 &a1) {
  return a0.push_back(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f227(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     const T1 &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f228(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     typename std::vector<T1, T2>::size_type a2, const T1 &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2>
void f229(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1,
          const T1 &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2>
using t85 = typename std::vector<T1, T2>::reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t86 = typename std::vector<T1, T2>::reverse_iterator::value_type;

template <typename T1, typename T2>
using t87 =
    typename std::vector<T1, T2>::const_reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t88 = typename std::vector<T1, T2>::const_reverse_iterator::value_type;
