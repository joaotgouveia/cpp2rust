#include <algorithm>
#include <initializer_list>
#include <utility>
#include <vector>

template <typename T1>
std::vector<bool, T1> f182(typename std::vector<bool, T1>::size_type a0) {
  return std::vector<bool, T1>(a0);
}

template <typename T1>
std::vector<bool, T1> f183(typename std::vector<bool, T1>::size_type a0,
                           const T1 &a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <typename T1>
std::vector<bool, T1> f184(typename std::vector<bool, T1>::size_type a0,
                           const bool &a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <typename T1>
std::vector<bool, T1> f185(typename std::vector<bool, T1>::size_type a0,
                           const bool &a1, const T1 &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <typename T1>
std::vector<bool, T1> f186(std::initializer_list<bool> a0) {
  return std::vector<bool, T1>(std::move(a0));
}

template <typename T1>
void f187(std::vector<bool, T1> &a0,
          typename std::vector<bool, T1>::size_type a1, const bool &a2) {
  return a0.assign(a1, a2);
}

template <typename T1>
typename std::vector<bool, T1>::size_type
f188(const std::vector<bool, T1> &a0) {
  return a0.max_size();
}

template <typename T1>
typename std::vector<bool, T1>::size_type
f189(const std::vector<bool, T1> &a0) {
  return a0.capacity();
}

template <typename T1>
typename std::vector<bool, T1>::size_type
f190(const std::vector<bool, T1> &a0) {
  return a0.size();
}

template <typename T1>
void f191(std::vector<bool, T1> &a0,
          typename std::vector<bool, T1>::size_type a1) {
  return a0.reserve(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator f192(std::vector<bool, T1> &a0) {
  return a0.begin();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f193(const std::vector<bool, T1> &a0) {
  return a0.begin();
}

template <typename T1>
typename std::vector<bool, T1>::iterator f194(std::vector<bool, T1> &a0) {
  return a0.end();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f195(const std::vector<bool, T1> &a0) {
  return a0.end();
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator
f196(std::vector<bool, T1> &a0) {
  return a0.rbegin();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f197(const std::vector<bool, T1> &a0) {
  return a0.rbegin();
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator
f198(std::vector<bool, T1> &a0) {
  return a0.rend();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f199(const std::vector<bool, T1> &a0) {
  return a0.rend();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f200(const std::vector<bool, T1> &a0) {
  return a0.cbegin();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f201(const std::vector<bool, T1> &a0) {
  return a0.cend();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f202(const std::vector<bool, T1> &a0) {
  return a0.crbegin();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f203(const std::vector<bool, T1> &a0) {
  return a0.crend();
}

template <typename T1>
typename std::vector<bool, T1>::reference
f204(std::vector<bool, T1> &a0, typename std::vector<bool, T1>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_reference
f205(const std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1>
typename std::vector<bool, T1>::reference
f206(std::vector<bool, T1> &a0, typename std::vector<bool, T1>::size_type a1) {
  return a0.at(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_reference
f207(const std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::size_type a1) {
  return a0.at(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reference f208(std::vector<bool, T1> &a0) {
  return a0.front();
}

template <typename T1>
typename std::vector<bool, T1>::const_reference
f209(const std::vector<bool, T1> &a0) {
  return a0.front();
}

template <typename T1>
typename std::vector<bool, T1>::reference f210(std::vector<bool, T1> &a0) {
  return a0.back();
}

template <typename T1>
typename std::vector<bool, T1>::const_reference
f211(const std::vector<bool, T1> &a0) {
  return a0.back();
}

template <typename T1> void f212(std::vector<bool, T1> &a0, const bool &a1) {
  return a0.push_back(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f213(std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::const_iterator a1, const bool &a2) {
  return a0.insert(a1, a2);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f214(std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::const_iterator a1,
     typename std::vector<bool, T1>::size_type a2, const bool &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f215(std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::const_iterator a1,
     std::initializer_list<bool> a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f216(std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f217(std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::const_iterator a1,
     typename std::vector<bool, T1>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1>
void f218(std::vector<bool, T1> &a0,
          typename std::vector<bool, T1>::size_type a1, bool a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T2>::value, int>
              T3 = 0>
std::vector<bool, T1> f219(T2 a0, T2 a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T2>::value, int>
              T3 = 0>
std::vector<bool, T1> f220(T2 a0, T2 a1, const T1 &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__has_forward_iterator_category<T2>::value, int>
        T3 = 0>
std::vector<bool, T1> f221(T2 a0, T2 a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__has_forward_iterator_category<T2>::value, int>
        T3 = 0>
std::vector<bool, T1> f222(T2 a0, T2 a1, const T1 &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <typename T1, typename T2,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T2>::value, int>
              T3 = 0>
void f223(std::vector<bool, T1> &a0, T2 a1, T2 a2) {
  return a0.template assign<T2>(a1, a2);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__has_forward_iterator_category<T2>::value, int>
        T3 = 0>
void f224(std::vector<bool, T1> &a0, T2 a1, T2 a2) {
  return a0.template assign<T2>(a1, a2);
}

template <typename T1, typename T2,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T2>::value, int>
              T3 = 0>
typename std::vector<bool, T1>::iterator
f225(std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::const_iterator a1, T2 a2, T2 a3) {
  return a0.template insert<T2>(a1, a2, a3);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__has_forward_iterator_category<T2>::value, int>
        T3 = 0>
typename std::vector<bool, T1>::iterator
f226(std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::const_iterator a1, T2 a2, T2 a3) {
  return a0.template insert<T2>(a1, a2, a3);
}

template <typename T1>
using t45 = typename std::vector<bool, T1>::pointer::difference_type;

template <typename T1>
using t46 = typename std::vector<bool, T1>::pointer::value_type;

template <typename T1>
using t47 = typename std::vector<bool, T1>::pointer::pointer;

template <typename T1>
using t48 = typename std::vector<bool, T1>::pointer::iterator_category;

template <typename T1>
typename std::vector<bool, T1>::pointer
f227(const std::__bit_iterator<std::vector<bool, T1>, false, 0> &a0) {
  return typename std::vector<bool, T1>::pointer(a0);
}

template <typename T1>
using t49 = typename std::vector<bool, T1>::const_pointer::difference_type;

template <typename T1>
using t50 = typename std::vector<bool, T1>::const_pointer::value_type;

template <typename T1>
using t51 = typename std::vector<bool, T1>::const_pointer::pointer;

template <typename T1>
using t52 = typename std::vector<bool, T1>::const_pointer::reference;

template <typename T1>
using t53 = typename std::vector<bool, T1>::const_pointer::iterator_category;

template <typename T1>
typename std::vector<bool, T1>::const_pointer
f228(const std::__bit_iterator<std::vector<bool, T1>, false, 0> &a0) {
  return typename std::vector<bool, T1>::const_pointer(a0);
}

template <typename T1>
using t54 = typename std::vector<bool, T1>::iterator::difference_type;

template <typename T1>
using t55 = typename std::vector<bool, T1>::iterator::value_type;

template <typename T1>
using t56 = typename std::vector<bool, T1>::iterator::iterator_category;

template <typename T1>
typename std::vector<bool, T1>::iterator
f229(const std::__bit_iterator<std::vector<bool, T1>, false, 0> &a0) {
  return typename std::vector<bool, T1>::iterator(a0);
}

template <typename T1>
using t57 = typename std::vector<bool, T1>::const_iterator::difference_type;

template <typename T1>
using t58 = typename std::vector<bool, T1>::const_iterator::value_type;

template <typename T1>
using t59 = typename std::vector<bool, T1>::const_iterator::iterator_category;

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f230(const std::__bit_iterator<std::vector<bool, T1>, false, 0> &a0) {
  return typename std::vector<bool, T1>::const_iterator(a0);
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, true, 0>
f231(const typename std::vector<bool, T1>::const_reference &a0) {
  return a0.operator&();
}

template <typename T1, typename T2>
using t60 = typename std::vector<T1, T2>::iterator;

template <typename T1, typename T2>
using t61 = typename std::vector<T1, T2>::const_iterator;

template <typename T1, typename T2>
using t62 = typename std::vector<T1, T2>::reverse_iterator;

template <typename T1, typename T2>
using t63 = typename std::vector<T1, T2>::const_reverse_iterator;

template <typename T1, typename T2>
std::vector<T1, T2> f232(typename std::vector<T1, T2>::size_type a0) {
  return std::vector<T1, T2>(a0);
}

template <typename T1, typename T2>
std::vector<T1, T2> f233(typename std::vector<T1, T2>::size_type a0,
                         const T2 &a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> f234(typename std::vector<T1, T2>::size_type a0,
                         const T1 &a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> f235(std::initializer_list<T1> a0) {
  return std::vector<T1, T2>(a0);
}

template <typename T1, typename T2>
void f236(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1,
          const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2> T2 f237(const std::vector<T1, T2> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::size_type f238(const std::vector<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::size_type f239(const std::vector<T1, T2> &a0) {
  return a0.capacity();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::size_type f240(const std::vector<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
void f241(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.reserve(a1);
}

template <typename T1, typename T2>
T1 &f242(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
const T1 &f243(const std::vector<T1, T2> &a0,
               typename std::vector<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
T1 &f244(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
const T1 &f245(const std::vector<T1, T2> &a0,
               typename std::vector<T1, T2>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2> T1 &f246(std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
const T1 &f247(const std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2> T1 &f248(std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
const T1 &f249(const std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f250(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     typename std::vector<T1, T2>::size_type a2, const T1 &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2>
void f251(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f252(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1,
          const T1 &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_allocator_v<T2>, int> T3 = 0>
std::vector<T1, T2> f253(typename std::vector<T1, T2>::size_type a0,
                         const T1 &a1, const T2 &a2) {
  return std::vector<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
std::vector<T1, T2> f254(T3 a0, T3 a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
std::vector<T1, T2> f255(T3 a0, T3 a1, const T2 &a2) {
  return std::vector<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_forward_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
std::vector<T1, T2> f256(T3 a0, T3 a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_forward_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
std::vector<T1, T2> f257(T3 a0, T3 a1, const T2 &a2) {
  return std::vector<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
void f258(std::vector<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_forward_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
void f259(std::vector<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
typename std::vector<T1, T2>::iterator
f260(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     T3 a2, T3 a3) {
  return a0.template insert<T3>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_forward_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
typename std::vector<T1, T2>::iterator
f261(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     T3 a2, T3 a3) {
  return a0.template insert<T3>(a1, a2, a3);
}
