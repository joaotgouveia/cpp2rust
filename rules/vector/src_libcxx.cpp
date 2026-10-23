#include <initializer_list>
#include <utility>
#include <vector>

template <typename T1>
std::vector<bool, T1> f167(typename std::vector<bool, T1>::size_type a0) {
  return std::vector<bool, T1>(a0);
}

template <typename T1>
std::vector<bool, T1> f168(typename std::vector<bool, T1>::size_type a0,
                           const T1 &a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <typename T1>
std::vector<bool, T1> f169(typename std::vector<bool, T1>::size_type a0,
                           const bool &a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <typename T1>
std::vector<bool, T1> f170(typename std::vector<bool, T1>::size_type a0,
                           const bool &a1, const T1 &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <typename T1>
std::vector<bool, T1> f171(std::initializer_list<bool> a0) {
  return std::vector<bool, T1>(std::move(a0));
}

template <typename T1>
void f172(std::vector<bool, T1> &a0,
          typename std::vector<bool, T1>::size_type a1, const bool &a2) {
  return a0.assign(a1, a2);
}

template <typename T1>
typename std::vector<bool, T1>::size_type
f173(const std::vector<bool, T1> &a0) {
  return a0.max_size();
}

template <typename T1>
typename std::vector<bool, T1>::size_type
f174(const std::vector<bool, T1> &a0) {
  return a0.capacity();
}

template <typename T1>
typename std::vector<bool, T1>::size_type
f175(const std::vector<bool, T1> &a0) {
  return a0.size();
}

template <typename T1>
void f176(std::vector<bool, T1> &a0,
          typename std::vector<bool, T1>::size_type a1) {
  return a0.reserve(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reference
f177(std::vector<bool, T1> &a0, typename std::vector<bool, T1>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_reference
f178(const std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1>
typename std::vector<bool, T1>::reference
f179(std::vector<bool, T1> &a0, typename std::vector<bool, T1>::size_type a1) {
  return a0.at(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_reference
f180(const std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::size_type a1) {
  return a0.at(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_reference
f181(const std::vector<bool, T1> &a0) {
  return a0.front();
}

template <typename T1>
typename std::vector<bool, T1>::const_reference
f182(const std::vector<bool, T1> &a0) {
  return a0.back();
}

template <typename T1> void f183(std::vector<bool, T1> &a0, const bool &a1) {
  return a0.push_back(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f184(std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::const_iterator a1,
     typename std::vector<bool, T1>::size_type a2, const bool &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1>
void f185(std::vector<bool, T1> &a0,
          typename std::vector<bool, T1>::size_type a1, bool a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T2>::value, int>
              T3 = 0>
std::vector<bool, T1> f186(T2 a0, T2 a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T2>::value, int>
              T3 = 0>
std::vector<bool, T1> f187(T2 a0, T2 a1, const T1 &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__has_forward_iterator_category<T2>::value, int>
        T3 = 0>
std::vector<bool, T1> f188(T2 a0, T2 a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__has_forward_iterator_category<T2>::value, int>
        T3 = 0>
std::vector<bool, T1> f189(T2 a0, T2 a1, const T1 &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <typename T1, typename T2,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T2>::value, int>
              T3 = 0>
void f190(std::vector<bool, T1> &a0, T2 a1, T2 a2) {
  return a0.template assign<T2>(a1, a2);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__has_forward_iterator_category<T2>::value, int>
        T3 = 0>
void f191(std::vector<bool, T1> &a0, T2 a1, T2 a2) {
  return a0.template assign<T2>(a1, a2);
}

template <typename T1, typename T2,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T2>::value, int>
              T3 = 0>
typename std::vector<bool, T1>::iterator
f192(std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::const_iterator a1, T2 a2, T2 a3) {
  return a0.template insert<T2>(a1, a2, a3);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__has_forward_iterator_category<T2>::value, int>
        T3 = 0>
typename std::vector<bool, T1>::iterator
f193(std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::const_iterator a1, T2 a2, T2 a3) {
  return a0.template insert<T2>(a1, a2, a3);
}

template <typename T1>
using t45 = typename std::vector<bool, T1>::pointer::difference_type;

template <typename T1>
using t46 = typename std::vector<bool, T1>::pointer::value_type;

template <typename T1>
using t47 = typename std::vector<bool, T1>::pointer::iterator_category;

template <typename T1>
using t48 = typename std::vector<bool, T1>::const_pointer::difference_type;

template <typename T1>
using t49 = typename std::vector<bool, T1>::const_pointer::value_type;

template <typename T1>
using t50 = typename std::vector<bool, T1>::const_pointer::iterator_category;

template <typename T1>
using t51 = typename std::vector<bool, T1>::iterator::difference_type;

template <typename T1>
using t52 = typename std::vector<bool, T1>::iterator::value_type;

template <typename T1>
using t53 = typename std::vector<bool, T1>::iterator::iterator_category;

template <typename T1>
using t54 = typename std::vector<bool, T1>::const_iterator::difference_type;

template <typename T1>
using t55 = typename std::vector<bool, T1>::const_iterator::value_type;

template <typename T1>
using t56 = typename std::vector<bool, T1>::const_iterator::iterator_category;

template <typename T1, typename T2>
std::vector<T1, T2> f194(typename std::vector<T1, T2>::size_type a0) {
  return std::vector<T1, T2>(a0);
}

template <typename T1, typename T2>
std::vector<T1, T2> f195(typename std::vector<T1, T2>::size_type a0,
                         const T2 &a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> f196(typename std::vector<T1, T2>::size_type a0,
                         const T1 &a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> f197(std::initializer_list<T1> a0) {
  return std::vector<T1, T2>(a0);
}

template <typename T1, typename T2>
void f198(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1,
          const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2> T2 f199(const std::vector<T1, T2> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::size_type f200(const std::vector<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::size_type f201(const std::vector<T1, T2> &a0) {
  return a0.capacity();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::size_type f202(const std::vector<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
void f203(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.reserve(a1);
}

template <typename T1, typename T2>
T1 &f204(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
const T1 &f205(const std::vector<T1, T2> &a0,
               typename std::vector<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
T1 &f206(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
const T1 &f207(const std::vector<T1, T2> &a0,
               typename std::vector<T1, T2>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2> T1 &f208(std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
const T1 &f209(const std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2> T1 &f210(std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
const T1 &f211(const std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f212(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     typename std::vector<T1, T2>::size_type a2, const T1 &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2>
void f213(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f214(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1,
          const T1 &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_allocator_v<T2>, int> T3 = 0>
std::vector<T1, T2> f215(typename std::vector<T1, T2>::size_type a0,
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
std::vector<T1, T2> f216(T3 a0, T3 a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
std::vector<T1, T2> f217(T3 a0, T3 a1, const T2 &a2) {
  return std::vector<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_forward_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
std::vector<T1, T2> f218(T3 a0, T3 a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_forward_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
std::vector<T1, T2> f219(T3 a0, T3 a1, const T2 &a2) {
  return std::vector<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
void f220(std::vector<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_forward_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
void f221(std::vector<T1, T2> &a0, T3 a1, T3 a2) {
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
f222(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
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
f223(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     T3 a2, T3 a3) {
  return a0.template insert<T3>(a1, a2, a3);
}
