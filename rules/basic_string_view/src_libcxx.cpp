#include <string_view>
#include <utility>

template <typename T1, typename T2> using t28 = std::basic_string_view<T1, T2>;

template <typename T1, typename T2>
using t29 = typename std::basic_string_view<T1, T2>::traits_type;

template <typename T1, typename T2>
using t30 = typename std::basic_string_view<T1, T2>::value_type;

template <typename T1, typename T2>
using t31 = typename std::basic_string_view<T1, T2>::pointer;

template <typename T1, typename T2>
using t32 = typename std::basic_string_view<T1, T2>::const_pointer;

template <typename T1, typename T2>
using t33 = typename std::basic_string_view<T1, T2>::reference;

template <typename T1, typename T2>
using t34 = typename std::basic_string_view<T1, T2>::const_reference;

template <typename T1, typename T2>
using t35 = typename std::basic_string_view<T1, T2>::const_iterator;

template <typename T1, typename T2>
using t36 = typename std::basic_string_view<T1, T2>::iterator;

template <typename T1, typename T2>
using t37 = typename std::basic_string_view<T1, T2>::const_reverse_iterator;

template <typename T1, typename T2>
using t38 = typename std::basic_string_view<T1, T2>::reverse_iterator;

template <typename T1, typename T2>
using t39 = typename std::basic_string_view<T1, T2>::size_type;

template <typename T1, typename T2>
using t40 = typename std::basic_string_view<T1, T2>::difference_type;

template <typename T1, typename T2> std::basic_string_view<T1, T2> f70() {
  return std::basic_string_view<T1, T2>();
}

template <typename T1, typename T2>
std::basic_string_view<T1, T2> f71(const std::basic_string_view<T1, T2> &a0) {
  return std::basic_string_view<T1, T2>(a0);
}

template <typename T1, typename T2>
std::basic_string_view<T1, T2> &f72(std::basic_string_view<T1, T2> &a0,
                                    const std::basic_string_view<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::basic_string_view<T1, T2>
f73(const T1 *a0, typename std::basic_string_view<T1, T2>::size_type a1) {
  return std::basic_string_view<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_string_view<T1, T2> f74(const T1 *a0) {
  return std::basic_string_view<T1, T2>(a0);
}

template <typename T1, typename T2>
const T1 *f75(const std::basic_string_view<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
const T1 *f76(const std::basic_string_view<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
const T1 *f77(const std::basic_string_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
const T1 *f78(const std::basic_string_view<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::const_reverse_iterator
f79(const std::basic_string_view<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::const_reverse_iterator
f80(const std::basic_string_view<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::const_reverse_iterator
f81(const std::basic_string_view<T1, T2> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::const_reverse_iterator
f82(const std::basic_string_view<T1, T2> &a0) {
  return a0.crend();
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f83(const std::basic_string_view<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f84(const std::basic_string_view<T1, T2> &a0) {
  return a0.length();
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f85(const std::basic_string_view<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
bool f86(const std::basic_string_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
const T1 &f87(const std::basic_string_view<T1, T2> &a0,
              typename std::basic_string_view<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
const T1 &f88(const std::basic_string_view<T1, T2> &a0,
              typename std::basic_string_view<T1, T2>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
const T1 &f89(const std::basic_string_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
const T1 &f90(const std::basic_string_view<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
const T1 *f91(const std::basic_string_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
void f92(std::basic_string_view<T1, T2> &a0,
         typename std::basic_string_view<T1, T2>::size_type a1) {
  return a0.remove_prefix(a1);
}

template <typename T1, typename T2>
void f93(std::basic_string_view<T1, T2> &a0,
         typename std::basic_string_view<T1, T2>::size_type a1) {
  return a0.remove_suffix(a1);
}

template <typename T1, typename T2>
void f94(std::basic_string_view<T1, T2> &a0,
         std::basic_string_view<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f95(const std::basic_string_view<T1, T2> &a0, T1 *a1,
    typename std::basic_string_view<T1, T2>::size_type a2,
    typename std::basic_string_view<T1, T2>::size_type a3) {
  return a0.copy(a1, a2, a3);
}

template <typename T1, typename T2>
std::basic_string_view<T1, T2>
f96(const std::basic_string_view<T1, T2> &a0,
    typename std::basic_string_view<T1, T2>::size_type a1,
    typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.substr(a1, a2);
}

template <typename T1, typename T2>
int f97(const std::basic_string_view<T1, T2> &a0,
        std::basic_string_view<T1, T2> a1) {
  return a0.compare(a1);
}

template <typename T1, typename T2>
int f98(const std::basic_string_view<T1, T2> &a0,
        typename std::basic_string_view<T1, T2>::size_type a1,
        typename std::basic_string_view<T1, T2>::size_type a2,
        std::basic_string_view<T1, T2> a3) {
  return a0.compare(a1, a2, a3);
}

template <typename T1, typename T2>
int f99(const std::basic_string_view<T1, T2> &a0,
        typename std::basic_string_view<T1, T2>::size_type a1,
        typename std::basic_string_view<T1, T2>::size_type a2,
        std::basic_string_view<T1, T2> a3,
        typename std::basic_string_view<T1, T2>::size_type a4,
        typename std::basic_string_view<T1, T2>::size_type a5) {
  return a0.compare(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
int f100(const std::basic_string_view<T1, T2> &a0, const T1 *a1) {
  return a0.compare(a1);
}

template <typename T1, typename T2>
int f101(const std::basic_string_view<T1, T2> &a0,
         typename std::basic_string_view<T1, T2>::size_type a1,
         typename std::basic_string_view<T1, T2>::size_type a2, const T1 *a3) {
  return a0.compare(a1, a2, a3);
}

template <typename T1, typename T2>
int f102(const std::basic_string_view<T1, T2> &a0,
         typename std::basic_string_view<T1, T2>::size_type a1,
         typename std::basic_string_view<T1, T2>::size_type a2, const T1 *a3,
         typename std::basic_string_view<T1, T2>::size_type a4) {
  return a0.compare(a1, a2, a3, a4);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f103(const std::basic_string_view<T1, T2> &a0,
     std::basic_string_view<T1, T2> a1,
     typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f104(const std::basic_string_view<T1, T2> &a0, T1 a1,
     typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f105(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
     typename std::basic_string_view<T1, T2>::size_type a2,
     typename std::basic_string_view<T1, T2>::size_type a3) {
  return a0.find(a1, a2, a3);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f106(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
     typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f107(const std::basic_string_view<T1, T2> &a0,
     std::basic_string_view<T1, T2> a1,
     typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f108(const std::basic_string_view<T1, T2> &a0, T1 a1,
     typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f109(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
     typename std::basic_string_view<T1, T2>::size_type a2,
     typename std::basic_string_view<T1, T2>::size_type a3) {
  return a0.rfind(a1, a2, a3);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f110(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
     typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f111(const std::basic_string_view<T1, T2> &a0,
     std::basic_string_view<T1, T2> a1,
     typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f112(const std::basic_string_view<T1, T2> &a0, T1 a1,
     typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f113(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
     typename std::basic_string_view<T1, T2>::size_type a2,
     typename std::basic_string_view<T1, T2>::size_type a3) {
  return a0.find_first_of(a1, a2, a3);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f114(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
     typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f115(const std::basic_string_view<T1, T2> &a0,
     std::basic_string_view<T1, T2> a1,
     typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f116(const std::basic_string_view<T1, T2> &a0, T1 a1,
     typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f117(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
     typename std::basic_string_view<T1, T2>::size_type a2,
     typename std::basic_string_view<T1, T2>::size_type a3) {
  return a0.find_last_of(a1, a2, a3);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f118(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
     typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f119(const std::basic_string_view<T1, T2> &a0,
     std::basic_string_view<T1, T2> a1,
     typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f120(const std::basic_string_view<T1, T2> &a0, T1 a1,
     typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f121(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
     typename std::basic_string_view<T1, T2>::size_type a2,
     typename std::basic_string_view<T1, T2>::size_type a3) {
  return a0.find_first_not_of(a1, a2, a3);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f122(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
     typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f123(const std::basic_string_view<T1, T2> &a0,
     std::basic_string_view<T1, T2> a1,
     typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f124(const std::basic_string_view<T1, T2> &a0, T1 a1,
     typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f125(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
     typename std::basic_string_view<T1, T2>::size_type a2,
     typename std::basic_string_view<T1, T2>::size_type a3) {
  return a0.find_last_not_of(a1, a2, a3);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f126(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
     typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2>
using t41 =
    typename std::basic_string_view<T1,
                                    T2>::const_reverse_iterator::iterator_type;

template <typename T1, typename T2>
using t42 = typename std::basic_string_view<
    T1, T2>::const_reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t43 =
    typename std::basic_string_view<T1, T2>::const_reverse_iterator::pointer;

template <typename T1, typename T2>
using t44 =
    typename std::basic_string_view<T1, T2>::const_reverse_iterator::value_type;

template <typename T1, typename T2>
using t45 = typename std::basic_string_view<
    T1, T2>::const_reverse_iterator::difference_type;

template <typename T1, typename T2>
using t46 =
    typename std::basic_string_view<T1, T2>::const_reverse_iterator::reference;

template <typename T1, typename T2>
using t47 =
    typename std::basic_string_view<T1, T2>::reverse_iterator::iterator_type;

template <typename T1, typename T2>
using t48 =
    typename std::basic_string_view<T1,
                                    T2>::reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t49 = typename std::basic_string_view<T1, T2>::reverse_iterator::pointer;

template <typename T1, typename T2>
using t50 =
    typename std::basic_string_view<T1, T2>::reverse_iterator::value_type;

template <typename T1, typename T2>
using t51 =
    typename std::basic_string_view<T1, T2>::reverse_iterator::difference_type;

template <typename T1, typename T2>
using t52 =
    typename std::basic_string_view<T1, T2>::reverse_iterator::reference;

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type f127() {
  return std::basic_string_view<T1, T2>::npos;
}
