#include <bitset>
#include <initializer_list>
#include <iterator>
#include <utility>
#include <vector>

template <typename T1>
std::vector<bool, T1> f133(typename std::vector<bool, T1>::size_type a0) {
  return std::vector<bool, T1>(std::move(a0));
}

template <typename T1>
std::vector<bool, T1> f134(typename std::vector<bool, T1>::size_type a0,
                           const T1 &a1) {
  return std::vector<bool, T1>(std::move(a0), a1);
}

template <typename T1>
std::vector<bool, T1> f135(typename std::vector<bool, T1>::size_type a0,
                           const bool &a1) {
  return std::vector<bool, T1>(std::move(a0), a1);
}

template <typename T1>
std::vector<bool, T1> f136(typename std::vector<bool, T1>::size_type a0,
                           const bool &a1, const T1 &a2) {
  return std::vector<bool, T1>(std::move(a0), a1, a2);
}

template <typename T1>
std::vector<bool, T1> f137(std::initializer_list<bool> a0) {
  return std::vector<bool, T1>(std::move(a0));
}

template <typename T1>
void f138(std::vector<bool, T1> &a0,
          typename std::vector<bool, T1>::size_type a1, const bool &a2) {
  return a0.assign(std::move(a1), a2);
}

template <typename T1>
typename std::vector<bool, T1>::size_type
f139(const std::vector<bool, T1> &a0) {
  return a0.max_size();
}

template <typename T1>
typename std::vector<bool, T1>::size_type
f140(const std::vector<bool, T1> &a0) {
  return a0.capacity();
}

template <typename T1>
typename std::vector<bool, T1>::size_type
f141(const std::vector<bool, T1> &a0) {
  return a0.size();
}

template <typename T1>
void f142(std::vector<bool, T1> &a0,
          typename std::vector<bool, T1>::size_type a1) {
  return a0.reserve(std::move(a1));
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, false>
f143(std::vector<bool, T1> &a0) {
  return a0.begin();
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, true>
f144(const std::vector<bool, T1> &a0) {
  return a0.begin();
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, false>
f145(std::vector<bool, T1> &a0) {
  return a0.end();
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, true>
f146(const std::vector<bool, T1> &a0) {
  return a0.end();
}

template <typename T1>
std::reverse_iterator<std::__bit_iterator<std::vector<bool, T1>, false>>
f147(std::vector<bool, T1> &a0) {
  return a0.rbegin();
}

template <typename T1>
std::reverse_iterator<std::__bit_iterator<std::vector<bool, T1>, true>>
f148(const std::vector<bool, T1> &a0) {
  return a0.rbegin();
}

template <typename T1>
std::reverse_iterator<std::__bit_iterator<std::vector<bool, T1>, false>>
f149(std::vector<bool, T1> &a0) {
  return a0.rend();
}

template <typename T1>
std::reverse_iterator<std::__bit_iterator<std::vector<bool, T1>, true>>
f150(const std::vector<bool, T1> &a0) {
  return a0.rend();
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, true>
f151(const std::vector<bool, T1> &a0) {
  return a0.cbegin();
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, true>
f152(const std::vector<bool, T1> &a0) {
  return a0.cend();
}

template <typename T1>
std::reverse_iterator<std::__bit_iterator<std::vector<bool, T1>, true>>
f153(const std::vector<bool, T1> &a0) {
  return a0.crbegin();
}

template <typename T1>
std::reverse_iterator<std::__bit_iterator<std::vector<bool, T1>, true>>
f154(const std::vector<bool, T1> &a0) {
  return a0.crend();
}

template <typename T1>
std::__bit_reference<std::vector<bool, T1>>
f155(std::vector<bool, T1> &a0, typename std::vector<bool, T1>::size_type a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
std::__bit_const_reference<std::vector<bool, T1>>
f156(const std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::size_type a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
std::__bit_reference<std::vector<bool, T1>>
f157(std::vector<bool, T1> &a0, typename std::vector<bool, T1>::size_type a1) {
  return a0.at(std::move(a1));
}

template <typename T1>
std::__bit_const_reference<std::vector<bool, T1>>
f158(const std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::size_type a1) {
  return a0.at(std::move(a1));
}

template <typename T1>
std::__bit_reference<std::vector<bool, T1>> f159(std::vector<bool, T1> &a0) {
  return a0.front();
}

template <typename T1>
std::__bit_const_reference<std::vector<bool, T1>>
f160(const std::vector<bool, T1> &a0) {
  return a0.front();
}

template <typename T1>
std::__bit_reference<std::vector<bool, T1>> f161(std::vector<bool, T1> &a0) {
  return a0.back();
}

template <typename T1>
std::__bit_const_reference<std::vector<bool, T1>>
f162(const std::vector<bool, T1> &a0) {
  return a0.back();
}

template <typename T1> void f163(std::vector<bool, T1> &a0, const bool &a1) {
  return a0.push_back(a1);
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, false>
f164(std::vector<bool, T1> &a0,
     std::__bit_iterator<std::vector<bool, T1>, true> a1, const bool &a2) {
  return a0.insert(std::move(a1), a2);
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, false>
f165(std::vector<bool, T1> &a0,
     std::__bit_iterator<std::vector<bool, T1>, true> a1,
     typename std::vector<bool, T1>::size_type a2, const bool &a3) {
  return a0.insert(std::move(a1), std::move(a2), a3);
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, false>
f166(std::vector<bool, T1> &a0,
     std::__bit_iterator<std::vector<bool, T1>, true> a1,
     std::initializer_list<bool> a2) {
  return a0.insert(std::move(a1), std::move(a2));
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, false>
f167(std::vector<bool, T1> &a0,
     std::__bit_iterator<std::vector<bool, T1>, true> a1) {
  return a0.erase(std::move(a1));
}

template <typename T1>
std::__bit_iterator<std::vector<bool, T1>, false>
f168(std::vector<bool, T1> &a0,
     std::__bit_iterator<std::vector<bool, T1>, true> a1,
     std::__bit_iterator<std::vector<bool, T1>, true> a2) {
  return a0.erase(std::move(a1), std::move(a2));
}

template <typename T1>
void f169(std::__bit_reference<std::vector<bool, T1>> a0,
          std::__bit_reference<std::vector<bool, T1>> a1) {
  return std::vector<bool, T1>::swap(std::move(a0), std::move(a1));
}

template <typename T1>
void f170(std::vector<bool, T1> &a0,
          typename std::vector<bool, T1>::size_type a1, bool a2) {
  return a0.resize(std::move(a1), a2);
}

template <typename T1, typename T2,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T2>::value, int>
              T3 = 0>
std::vector<bool, T1> f171(T2 a0, T2 a1) {
  return std::vector<bool, T1>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T2>::value, int>
              T3 = 0>
std::vector<bool, T1> f172(T2 a0, T2 a1, const T1 &a2) {
  return std::vector<bool, T1>(std::move(a0), std::move(a1), a2);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__has_forward_iterator_category<T2>::value, int>
        T3 = 0>
std::vector<bool, T1> f173(T2 a0, T2 a1) {
  return std::vector<bool, T1>(std::move(a0), std::move(a1));
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__has_forward_iterator_category<T2>::value, int>
        T3 = 0>
std::vector<bool, T1> f174(T2 a0, T2 a1, const T1 &a2) {
  return std::vector<bool, T1>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T2>::value, int>
              T3 = 0>
void f175(std::vector<bool, T1> &a0, T2 a1, T2 a2) {
  return a0.template assign<T2>(std::move(a1), std::move(a2));
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__has_forward_iterator_category<T2>::value, int>
        T3 = 0>
void f176(std::vector<bool, T1> &a0, T2 a1, T2 a2) {
  return a0.template assign<T2>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T2>::value, int>
              T3 = 0>
std::__bit_iterator<std::vector<bool, T1>, false>
f177(std::vector<bool, T1> &a0,
     std::__bit_iterator<std::vector<bool, T1>, true> a1, T2 a2, T2 a3) {
  return a0.template insert<T2>(std::move(a1), std::move(a2), std::move(a3));
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__has_forward_iterator_category<T2>::value, int>
        T3 = 0>
std::__bit_iterator<std::vector<bool, T1>, false>
f178(std::vector<bool, T1> &a0,
     std::__bit_iterator<std::vector<bool, T1>, true> a1, T2 a2, T2 a3) {
  return a0.template insert<T2>(std::move(a1), std::move(a2), std::move(a3));
}

template <typename T1, typename T2>
std::vector<T1, T2> f179(typename std::vector<T1, T2>::size_type a0) {
  return std::vector<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::vector<T1, T2> f180(typename std::vector<T1, T2>::size_type a0,
                         const T2 &a1) {
  return std::vector<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> f181(typename std::vector<T1, T2>::size_type a0,
                         const T1 &a1) {
  return std::vector<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> f182(std::initializer_list<T1> a0) {
  return std::vector<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
void f183(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1,
          const T1 &a2) {
  return a0.assign(std::move(a1), a2);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::size_type f184(const std::vector<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::size_type f185(const std::vector<T1, T2> &a0) {
  return a0.capacity();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::size_type f186(const std::vector<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
void f187(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.reserve(std::move(a1));
}

template <typename T1, typename T2>
T1 &f188(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2>
const T1 &f189(const std::vector<T1, T2> &a0,
               typename std::vector<T1, T2>::size_type a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2>
T1 &f190(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.at(std::move(a1));
}

template <typename T1, typename T2>
const T1 &f191(const std::vector<T1, T2> &a0,
               typename std::vector<T1, T2>::size_type a1) {
  return a0.at(std::move(a1));
}

template <typename T1, typename T2> T1 &f192(std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
const T1 &f193(const std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2> T1 &f194(std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
const T1 &f195(const std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f196(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     typename std::vector<T1, T2>::size_type a2, const T1 &a3) {
  return a0.insert(std::move(a1), std::move(a2), a3);
}

template <typename T1, typename T2>
void f197(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.resize(std::move(a1));
}

template <typename T1, typename T2>
void f198(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1,
          const T1 &a2) {
  return a0.resize(std::move(a1), a2);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_allocator_v<T2>, int> T3 = 0>
std::vector<T1, T2> f199(typename std::vector<T1, T2>::size_type a0,
                         const T1 &a1, const T2 &a2) {
  return std::vector<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
std::vector<T1, T2> f200(T3 a0, T3 a1) {
  return std::vector<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
std::vector<T1, T2> f201(T3 a0, T3 a1, const T2 &a2) {
  return std::vector<T1, T2>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_forward_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
std::vector<T1, T2> f202(T3 a0, T3 a1) {
  return std::vector<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_forward_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
std::vector<T1, T2> f203(T3 a0, T3 a1, const T2 &a2) {
  return std::vector<T1, T2>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
void f204(std::vector<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_forward_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
void f205(std::vector<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
typename std::vector<T1, T2>::iterator
f206(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     T3 a2, T3 a3) {
  return a0.template insert<T3>(std::move(a1), std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_forward_iterator_category<T3>::value &&
                  std::is_constructible<
                      T1, typename std::iterator_traits<T3>::reference>::value,
              int>
              T4 = 0>
typename std::vector<T1, T2>::iterator
f207(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     T3 a2, T3 a3) {
  return a0.template insert<T3>(std::move(a1), std::move(a2), std::move(a3));
}
