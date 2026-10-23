#include <algorithm>
#include <initializer_list>
#include <utility>
#include <vector>

template <typename T1> using t43 = typename std::vector<bool, T1>::reference;

template <typename T1>
using t44 = typename std::vector<bool, T1>::const_reference;

template <typename T1> using t45 = typename std::vector<bool, T1>::pointer;

template <typename T1>
using t46 = typename std::vector<bool, T1>::const_pointer;

template <typename T1> using t47 = typename std::vector<bool, T1>::iterator;

template <typename T1>
using t48 = typename std::vector<bool, T1>::const_iterator;

template <typename T1>
using t49 = typename std::vector<bool, T1>::const_reverse_iterator;

template <typename T1>
using t50 = typename std::vector<bool, T1>::reverse_iterator;

template <typename T1>
std::vector<bool, T1>
f91(typename std::vector<bool, T1>::size_type a0, const bool &a1,
    const typename std::vector<bool, T1>::allocator_type &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <typename T1>
std::vector<bool, T1>
f92(std::initializer_list<bool> a0,
    const typename std::vector<bool, T1>::allocator_type &a1) {
  return std::vector<bool, T1>(std::move(a0), a1);
}

template <typename T1>
std::vector<bool, T1> &f93(std::vector<bool, T1> &a0,
                           std::initializer_list<bool> a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
void f94(std::vector<bool, T1> &a0,
         typename std::vector<bool, T1>::size_type a1, const bool &a2) {
  return a0.assign(a1, a2);
}

template <typename T1>
void f95(std::vector<bool, T1> &a0, std::initializer_list<bool> a1) {
  return a0.assign(std::move(a1));
}

template <typename T1> void f96(std::vector<bool, T1> &a0, bool a1) {
  return a0.push_back(a1);
}

template <typename T1>
void f97(typename std::vector<bool, T1>::reference a0,
         typename std::vector<bool, T1>::reference a1) {
  return std::vector<bool, T1>::swap(a0, a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f98(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1, const bool &a2) {
  return a0.insert(a1, a2);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f99(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1,
    typename std::vector<bool, T1>::size_type a2, const bool &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f100(std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::const_iterator a1,
     std::initializer_list<bool> a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1>
void f101(std::vector<bool, T1> &a0,
          typename std::vector<bool, T1>::size_type a1, bool a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2,
          typename T3 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T2>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
std::vector<bool, T1>
f102(T2 a0, T2 a1, const typename std::vector<bool, T1>::allocator_type &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <typename T1, typename T2,
          typename T3 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T2>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
void f103(std::vector<bool, T1> &a0, T2 a1, T2 a2) {
  return a0.template assign<T2>(a1, a2);
}

template <typename T1, typename T2,
          typename T3 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T2>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
typename std::vector<bool, T1>::iterator
f104(std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::const_iterator a1, T2 a2, T2 a3) {
  return a0.template insert<T2>(a1, a2, a3);
}

template <typename T1, typename... T2>
typename std::vector<bool, T1>::iterator
f105(std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::const_iterator a1, T2 &&...a2) {
  return a0.template emplace<T2...>(a1, std::forward<T2>(a2)...);
}

template <typename T1>
typename std::vector<bool, T1>::reference
f106(const typename std::vector<bool, T1>::reference &a0) {
  return typename std::vector<bool, T1>::reference(a0);
}

template <typename T1>
bool f107(const typename std::vector<bool, T1>::reference &a0) {
  return static_cast<bool>(a0);
}

template <typename T1>
typename std::vector<bool, T1>::reference &
f108(typename std::vector<bool, T1>::reference &a0, bool a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reference &
f109(typename std::vector<bool, T1>::reference &a0,
     const typename std::vector<bool, T1>::reference &a1) {
  return a0.operator=(a1);
}

template <typename T1>
bool f110(const typename std::vector<bool, T1>::reference &a0,
          const typename std::vector<bool, T1>::reference &a1) {
  return a0.operator==(a1);
}

template <typename T1>
bool f111(const typename std::vector<bool, T1>::reference &a0,
          const typename std::vector<bool, T1>::reference &a1) {
  return a0.operator<(a1);
}

template <typename T1>
void f112(typename std::vector<bool, T1>::reference &a0) {
  return a0.flip();
}

template <typename T1>
void f113(bool &a0, typename std::vector<bool, T1>::reference a1) {
  return swap(a0, a1);
}

template <typename T1>
void f114(typename std::vector<bool, T1>::reference a0, bool &a1) {
  return swap(a0, a1);
}

template <typename T1>
void f115(typename std::vector<bool, T1>::reference a0,
          typename std::vector<bool, T1>::reference a1) {
  return swap(a0, a1);
}

template <typename T1>
using t51 = typename std::vector<bool, T1>::iterator::reference;

template <typename T1>
using t52 = typename std::vector<bool, T1>::iterator::pointer;

template <typename T1>
using t53 = typename std::vector<bool, T1>::iterator::iterator;

template <typename T1> typename std::vector<bool, T1>::iterator f116() {
  return typename std::vector<bool, T1>::iterator();
}

template <typename T1>
typename std::vector<bool, T1>::iterator f117(unsigned long *a0,
                                              unsigned int a1) {
  return typename std::vector<bool, T1>::iterator(a0, a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f118(typename std::vector<bool, T1>::iterator &a0) {
  return a0.operator++();
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f119(typename std::vector<bool, T1>::iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f120(typename std::vector<bool, T1>::iterator &a0) {
  return a0.operator--();
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f121(typename std::vector<bool, T1>::iterator &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f122(typename std::vector<bool, T1>::iterator &a0,
     typename std::vector<bool, T1>::iterator::difference_type a1) {
  return a0.operator+=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f123(typename std::vector<bool, T1>::iterator &a0,
     typename std::vector<bool, T1>::iterator::difference_type a1) {
  return a0.operator-=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f124(const typename std::vector<bool, T1>::iterator &a0) {
  return typename std::vector<bool, T1>::iterator(a0);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f125(typename std::vector<bool, T1>::iterator &&a0) {
  return typename std::vector<bool, T1>::iterator(std::move(a0));
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f126(typename std::vector<bool, T1>::iterator &a0,
     const typename std::vector<bool, T1>::iterator &a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f127(typename std::vector<bool, T1>::iterator &a0,
     typename std::vector<bool, T1>::iterator &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
using t54 = typename std::vector<bool, T1>::const_iterator::reference;

template <typename T1>
using t55 = typename std::vector<bool, T1>::const_iterator::const_reference;

template <typename T1>
using t56 = typename std::vector<bool, T1>::const_iterator::pointer;

template <typename T1>
using t57 = typename std::vector<bool, T1>::const_iterator::const_iterator;

template <typename T1> typename std::vector<bool, T1>::const_iterator f128() {
  return typename std::vector<bool, T1>::const_iterator();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator f129(unsigned long *a0,
                                                    unsigned int a1) {
  return typename std::vector<bool, T1>::const_iterator(a0, a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f130(typename std::vector<bool, T1>::const_iterator &a0) {
  return a0.operator++();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f131(typename std::vector<bool, T1>::const_iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f132(typename std::vector<bool, T1>::const_iterator &a0) {
  return a0.operator--();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f133(typename std::vector<bool, T1>::const_iterator &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f134(typename std::vector<bool, T1>::const_iterator &a0,
     typename std::vector<bool, T1>::const_iterator::difference_type a1) {
  return a0.operator+=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f135(typename std::vector<bool, T1>::const_iterator &a0,
     typename std::vector<bool, T1>::const_iterator::difference_type a1) {
  return a0.operator-=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f136(const typename std::vector<bool, T1>::const_iterator &a0) {
  return typename std::vector<bool, T1>::const_iterator(a0);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f137(typename std::vector<bool, T1>::const_iterator &&a0) {
  return typename std::vector<bool, T1>::const_iterator(std::move(a0));
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f138(typename std::vector<bool, T1>::const_iterator &a0,
     const typename std::vector<bool, T1>::const_iterator &a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f139(typename std::vector<bool, T1>::const_iterator &a0,
     typename std::vector<bool, T1>::const_iterator &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator f140() {
  return typename std::vector<bool, T1>::const_reverse_iterator();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f141(const typename std::vector<bool, T1>::const_reverse_iterator &a0) {
  return typename std::vector<bool, T1>::const_reverse_iterator(a0);
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator &
f142(typename std::vector<bool, T1>::const_reverse_iterator &a0,
     const typename std::vector<bool, T1>::const_reverse_iterator &a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator::reference
f143(const typename std::vector<bool, T1>::const_reverse_iterator &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator::pointer
f144(const typename std::vector<bool, T1>::const_reverse_iterator &a0) {
  return a0.operator->();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator &
f145(typename std::vector<bool, T1>::const_reverse_iterator &a0) {
  return a0.operator++();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f146(typename std::vector<bool, T1>::const_reverse_iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator &
f147(typename std::vector<bool, T1>::const_reverse_iterator &a0) {
  return a0.operator--();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f148(typename std::vector<bool, T1>::const_reverse_iterator &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f149(const typename std::vector<bool, T1>::const_reverse_iterator &a0,
     typename std::vector<bool, T1>::const_reverse_iterator::difference_type
         a1) {
  return a0.operator+(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator &
f150(typename std::vector<bool, T1>::const_reverse_iterator &a0,
     typename std::vector<bool, T1>::const_reverse_iterator::difference_type
         a1) {
  return a0.operator+=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f151(const typename std::vector<bool, T1>::const_reverse_iterator &a0,
     typename std::vector<bool, T1>::const_reverse_iterator::difference_type
         a1) {
  return a0.operator-(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator &
f152(typename std::vector<bool, T1>::const_reverse_iterator &a0,
     typename std::vector<bool, T1>::const_reverse_iterator::difference_type
         a1) {
  return a0.operator-=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator::reference
f153(const typename std::vector<bool, T1>::const_reverse_iterator &a0,
     typename std::vector<bool, T1>::const_reverse_iterator::difference_type
         a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::vector<bool, T1>::const_reverse_iterator
f154(const std::reverse_iterator<T2> &a0) {
  return typename std::vector<bool, T1>::const_reverse_iterator(a0);
}

template <typename T1, typename T2>
typename std::vector<bool, T1>::const_reverse_iterator &
f155(typename std::vector<bool, T1>::const_reverse_iterator &a0,
     const std::reverse_iterator<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1> typename std::vector<bool, T1>::reverse_iterator f156() {
  return typename std::vector<bool, T1>::reverse_iterator();
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator
f157(const typename std::vector<bool, T1>::reverse_iterator &a0) {
  return typename std::vector<bool, T1>::reverse_iterator(a0);
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator &
f158(typename std::vector<bool, T1>::reverse_iterator &a0,
     const typename std::vector<bool, T1>::reverse_iterator &a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator::reference
f159(const typename std::vector<bool, T1>::reverse_iterator &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator::pointer
f160(const typename std::vector<bool, T1>::reverse_iterator &a0) {
  return a0.operator->();
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator &
f161(typename std::vector<bool, T1>::reverse_iterator &a0) {
  return a0.operator++();
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator
f162(typename std::vector<bool, T1>::reverse_iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator &
f163(typename std::vector<bool, T1>::reverse_iterator &a0) {
  return a0.operator--();
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator
f164(typename std::vector<bool, T1>::reverse_iterator &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator
f165(const typename std::vector<bool, T1>::reverse_iterator &a0,
     typename std::vector<bool, T1>::reverse_iterator::difference_type a1) {
  return a0.operator+(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator &
f166(typename std::vector<bool, T1>::reverse_iterator &a0,
     typename std::vector<bool, T1>::reverse_iterator::difference_type a1) {
  return a0.operator+=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator
f167(const typename std::vector<bool, T1>::reverse_iterator &a0,
     typename std::vector<bool, T1>::reverse_iterator::difference_type a1) {
  return a0.operator-(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator &
f168(typename std::vector<bool, T1>::reverse_iterator &a0,
     typename std::vector<bool, T1>::reverse_iterator::difference_type a1) {
  return a0.operator-=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator::reference
f169(const typename std::vector<bool, T1>::reverse_iterator &a0,
     typename std::vector<bool, T1>::reverse_iterator::difference_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::vector<bool, T1>::reverse_iterator
f170(const std::reverse_iterator<T2> &a0) {
  return typename std::vector<bool, T1>::reverse_iterator(a0);
}

template <typename T1, typename T2>
typename std::vector<bool, T1>::reverse_iterator &
f171(typename std::vector<bool, T1>::reverse_iterator &a0,
     const std::reverse_iterator<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
std::vector<T1, T2>
f172(typename std::vector<T1, T2>::size_type a0,
     const typename std::vector<T1, T2>::value_type &a1,
     const typename std::vector<T1, T2>::allocator_type &a2) {
  return std::vector<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
void f173(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1,
          const typename std::vector<T1, T2>::value_type &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
void f174(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1,
          const typename std::vector<T1, T2>::value_type &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference
f175(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f176(const std::vector<T1, T2> &a0,
     typename std::vector<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference
f177(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f178(const std::vector<T1, T2> &a0,
     typename std::vector<T1, T2>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f179(std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f180(const std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f181(std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f182(const std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2> T1 *f183(std::vector<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
const T1 *f184(const std::vector<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
void f185(std::vector<T1, T2> &a0,
          const typename std::vector<T1, T2>::value_type &a1) {
  return a0.push_back(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f186(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     const typename std::vector<T1, T2>::value_type &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f187(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     typename std::vector<T1, T2>::size_type a2,
     const typename std::vector<T1, T2>::value_type &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
std::vector<T1, T2>
f188(T3 a0, T3 a1, const typename std::vector<T1, T2>::allocator_type &a2) {
  return std::vector<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
void f189(std::vector<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
typename std::vector<T1, T2>::iterator
f190(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     T3 a2, T3 a3) {
  return a0.template insert<T3>(a1, a2, a3);
}

template <typename T1, typename T2>
using t58 = typename std::vector<T1, T2>::iterator::iterator_type;

template <typename T1, typename T2>
using t59 = typename std::vector<T1, T2>::const_iterator::iterator_type;
