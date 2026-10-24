#include <algorithm>
#include <bitset>
#include <initializer_list>
#include <iterator>
#include <utility>
#include <vector>

template <typename T1>
std::vector<bool, T1> f137(typename std::vector<bool, T1>::size_type a0) {
  return std::vector<bool, T1>(a0);
}

template <typename T1>
std::vector<bool, T1> f138(typename std::vector<bool, T1>::size_type a0,
                           const T1 &a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <typename T1>
std::vector<bool, T1> f139(typename std::vector<bool, T1>::size_type a0,
                           const bool &a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <typename T1>
std::vector<bool, T1> f140(typename std::vector<bool, T1>::size_type a0,
                           const bool &a1, const T1 &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <typename T1>
std::vector<bool, T1> f141(std::initializer_list<bool> a0) {
  return std::vector<bool, T1>(std::move(a0));
}

template <typename T1>
void f142(std::vector<bool, T1> &a0,
          typename std::vector<bool, T1>::size_type a1, const bool &a2) {
  return a0.assign(a1, a2);
}

template <typename T1>
typename std::vector<bool, T1>::size_type
f143(const std::vector<bool, T1> &a0) {
  return a0.max_size();
}

template <typename T1>
typename std::vector<bool, T1>::size_type
f144(const std::vector<bool, T1> &a0) {
  return a0.capacity();
}

template <typename T1>
typename std::vector<bool, T1>::size_type
f145(const std::vector<bool, T1> &a0) {
  return a0.size();
}

template <typename T1>
void f146(std::vector<bool, T1> &a0,
          typename std::vector<bool, T1>::size_type a1) {
  return a0.reserve(a1);
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, false>
f147(std::vector<bool, T1> &a0) {
  return a0.begin();
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, true>
f148(const std::vector<bool, T1> &a0) {
  return a0.begin();
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, false>
f149(std::vector<bool, T1> &a0) {
  return a0.end();
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, true>
f150(const std::vector<bool, T1> &a0) {
  return a0.end();
}

template <typename T1>
std::reverse_iterator<std::__bit_iterator<std::vector<bool, T1>, false>>
f151(std::vector<bool, T1> &a0) {
  return a0.rbegin();
}

template <typename T1>
std::reverse_iterator<std::__bit_iterator<std::vector<bool, T1>, true>>
f152(const std::vector<bool, T1> &a0) {
  return a0.rbegin();
}

template <typename T1>
std::reverse_iterator<std::__bit_iterator<std::vector<bool, T1>, false>>
f153(std::vector<bool, T1> &a0) {
  return a0.rend();
}

template <typename T1>
std::reverse_iterator<std::__bit_iterator<std::vector<bool, T1>, true>>
f154(const std::vector<bool, T1> &a0) {
  return a0.rend();
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, true>
f155(const std::vector<bool, T1> &a0) {
  return a0.cbegin();
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, true>
f156(const std::vector<bool, T1> &a0) {
  return a0.cend();
}

template <typename T1>
std::reverse_iterator<std::__bit_iterator<std::vector<bool, T1>, true>>
f157(const std::vector<bool, T1> &a0) {
  return a0.crbegin();
}

template <typename T1>
std::reverse_iterator<std::__bit_iterator<std::vector<bool, T1>, true>>
f158(const std::vector<bool, T1> &a0) {
  return a0.crend();
}

template <typename T1>
std::__bit_reference<std::vector<bool, T1>>
f159(std::vector<bool, T1> &a0, typename std::vector<bool, T1>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::__bit_const_reference<std::vector<bool, T1>>
f160(const std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::__bit_reference<std::vector<bool, T1>>
f161(std::vector<bool, T1> &a0, typename std::vector<bool, T1>::size_type a1) {
  return a0.at(a1);
}

template <typename T1>
std::__bit_const_reference<std::vector<bool, T1>>
f162(const std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::size_type a1) {
  return a0.at(a1);
}

template <typename T1>
std::__bit_reference<std::vector<bool, T1>> f163(std::vector<bool, T1> &a0) {
  return a0.front();
}

template <typename T1>
std::__bit_const_reference<std::vector<bool, T1>>
f164(const std::vector<bool, T1> &a0) {
  return a0.front();
}

template <typename T1>
std::__bit_reference<std::vector<bool, T1>> f165(std::vector<bool, T1> &a0) {
  return a0.back();
}

template <typename T1>
std::__bit_const_reference<std::vector<bool, T1>>
f166(const std::vector<bool, T1> &a0) {
  return a0.back();
}

template <typename T1> void f167(std::vector<bool, T1> &a0, const bool &a1) {
  return a0.push_back(a1);
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, false>
f168(std::vector<bool, T1> &a0,
     std::__bit_iterator<std::vector<bool, T1>, true> a1, const bool &a2) {
  return a0.insert(a1, a2);
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, false>
f169(std::vector<bool, T1> &a0,
     std::__bit_iterator<std::vector<bool, T1>, true> a1,
     typename std::vector<bool, T1>::size_type a2, const bool &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, false>
f170(std::vector<bool, T1> &a0,
     std::__bit_iterator<std::vector<bool, T1>, true> a1,
     std::initializer_list<bool> a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, false>
f171(std::vector<bool, T1> &a0,
     std::__bit_iterator<std::vector<bool, T1>, true> a1) {
  return a0.erase(a1);
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, false>
f172(std::vector<bool, T1> &a0,
     std::__bit_iterator<std::vector<bool, T1>, true> a1,
     std::__bit_iterator<std::vector<bool, T1>, true> a2) {
  return a0.erase(a1, a2);
}

template <typename T1>
void f173(std::__bit_reference<std::vector<bool, T1>> a0,
          std::__bit_reference<std::vector<bool, T1>> a1) {
  return std::vector<bool, T1>::swap(a0, a1);
}

template <typename T1>
void f174(std::vector<bool, T1> &a0,
          typename std::vector<bool, T1>::size_type a1, bool a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T2>::value, int>
              T3 = 0>
std::vector<bool, T1> f175(T2 a0, T2 a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T2>::value, int>
              T3 = 0>
std::vector<bool, T1> f176(T2 a0, T2 a1, const T1 &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__has_forward_iterator_category<T2>::value, int>
        T3 = 0>
std::vector<bool, T1> f177(T2 a0, T2 a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__has_forward_iterator_category<T2>::value, int>
        T3 = 0>
std::vector<bool, T1> f178(T2 a0, T2 a1, const T1 &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <typename T1, typename T2,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T2>::value, int>
              T3 = 0>
void f179(std::vector<bool, T1> &a0, T2 a1, T2 a2) {
  return a0.template assign<T2>(a1, a2);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__has_forward_iterator_category<T2>::value, int>
        T3 = 0>
void f180(std::vector<bool, T1> &a0, T2 a1, T2 a2) {
  return a0.template assign<T2>(a1, a2);
}

template <typename T1, typename T2,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T2>::value, int>
              T3 = 0>
std::__bit_iterator<std::vector<bool, T1>, false>
f181(std::vector<bool, T1> &a0,
     std::__bit_iterator<std::vector<bool, T1>, true> a1, T2 a2, T2 a3) {
  return a0.template insert<T2>(a1, a2, a3);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__has_forward_iterator_category<T2>::value, int>
        T3 = 0>
std::__bit_iterator<std::vector<bool, T1>, false>
f182(std::vector<bool, T1> &a0,
     std::__bit_iterator<std::vector<bool, T1>, true> a1, T2 a2, T2 a3) {
  return a0.template insert<T2>(a1, a2, a3);
}

template <typename T1>
using t42 = typename std::vector<bool, T1>::pointer::difference_type;

template <typename T1>
using t43 = typename std::vector<bool, T1>::pointer::value_type;

template <typename T1>
using t44 = typename std::vector<bool, T1>::pointer::pointer;

template <typename T1>
using t45 = typename std::vector<bool, T1>::pointer::reference;

template <typename T1>
using t46 = typename std::vector<bool, T1>::pointer::iterator_category;

template <typename T1>
typename std::vector<bool, T1>::pointer
f183(const std::__bit_iterator<std::vector<bool, T1>, false> &a0) {
  return typename std::vector<bool, T1>::pointer(a0);
}

template <typename T1>
using t47 = typename std::vector<bool, T1>::const_pointer::difference_type;

template <typename T1>
using t48 = typename std::vector<bool, T1>::const_pointer::value_type;

template <typename T1>
using t49 = typename std::vector<bool, T1>::const_pointer::pointer;

template <typename T1>
using t50 = typename std::vector<bool, T1>::const_pointer::reference;

template <typename T1>
using t51 = typename std::vector<bool, T1>::const_pointer::iterator_category;

template <typename T1>
typename std::vector<bool, T1>::const_pointer
f184(const std::__bit_iterator<std::vector<bool, T1>, false> &a0) {
  return typename std::vector<bool, T1>::const_pointer(a0);
}

template <typename T1>
typename std::vector<bool, T1>::const_reference
f185(const std::__bit_reference<std::vector<bool, T1>> &a0) {
  return typename std::vector<bool, T1>::const_reference(a0);
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, true>
f186(const typename std::vector<bool, T1>::const_reference &a0) {
  return a0.operator&();
}

template <typename T1, typename T2>
std::vector<T1, T2> f187(typename std::vector<T1, T2>::size_type a0) {
  return std::vector<T1, T2>(a0);
}

template <typename T1, typename T2>
std::vector<T1, T2> f188(typename std::vector<T1, T2>::size_type a0,
                         const T2 &a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> f189(typename std::vector<T1, T2>::size_type a0,
                         const T1 &a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> f190(std::initializer_list<T1> a0) {
  return std::vector<T1, T2>(a0);
}

template <typename T1, typename T2>
void f191(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1,
          const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::size_type f192(const std::vector<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::size_type f193(const std::vector<T1, T2> &a0) {
  return a0.capacity();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::size_type f194(const std::vector<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
void f195(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.reserve(a1);
}

template <typename T1, typename T2>
T1 &f196(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
const T1 &f197(const std::vector<T1, T2> &a0,
               typename std::vector<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
T1 &f198(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
const T1 &f199(const std::vector<T1, T2> &a0,
               typename std::vector<T1, T2>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2> T1 &f200(std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
const T1 &f201(const std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2> T1 &f202(std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
const T1 &f203(const std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f204(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     typename std::vector<T1, T2>::size_type a2, const T1 &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2>
void f205(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f206(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1,
          const T1 &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_allocator_v<T2>, int> T3 = 0>
std::vector<T1, T2> f207(typename std::vector<T1, T2>::size_type a0,
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
std::vector<T1, T2> f208(T3 a0, T3 a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
std::vector<T1, T2> f209(T3 a0, T3 a1, const T2 &a2) {
  return std::vector<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_forward_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
std::vector<T1, T2> f210(T3 a0, T3 a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_forward_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
std::vector<T1, T2> f211(T3 a0, T3 a1, const T2 &a2) {
  return std::vector<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
void f212(std::vector<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_forward_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
void f213(std::vector<T1, T2> &a0, T3 a1, T3 a2) {
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
f214(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
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
f215(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     T3 a2, T3 a3) {
  return a0.template insert<T3>(a1, a2, a3);
}
