#include <cstddef>
#include <iterator>
#include <string_view>
#include <utility>

template <typename T1, typename T2> using t14 = std::basic_string_view<T1, T2>;

template <typename T1, typename T2>
using t15 = typename std::basic_string_view<T1, T2>::traits_type;

template <typename T1, typename T2>
using t16 = typename std::basic_string_view<T1, T2>::value_type;

template <typename T1, typename T2>
using t17 = typename std::basic_string_view<T1, T2>::pointer;

template <typename T1, typename T2>
using t18 = typename std::basic_string_view<T1, T2>::const_pointer;

template <typename T1, typename T2>
using t19 = typename std::basic_string_view<T1, T2>::reference;

template <typename T1, typename T2>
using t20 = typename std::basic_string_view<T1, T2>::const_reference;

template <typename T1, typename T2>
using t21 = typename std::basic_string_view<T1, T2>::const_iterator;

template <typename T1, typename T2>
using t22 = typename std::basic_string_view<T1, T2>::iterator;

template <typename T1, typename T2>
using t23 = typename std::basic_string_view<T1, T2>::const_reverse_iterator;

template <typename T1, typename T2>
using t24 = typename std::basic_string_view<T1, T2>::reverse_iterator;

template <typename T1, typename T2>
using t25 = typename std::basic_string_view<T1, T2>::size_type;

template <typename T1, typename T2>
using t26 = typename std::basic_string_view<T1, T2>::difference_type;

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
std::basic_string_view<T1, T2> f73(const T1 *a0, std::size_t a1) {
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
std::reverse_iterator<const T1 *>
f79(const std::basic_string_view<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2>
std::reverse_iterator<const T1 *>
f80(const std::basic_string_view<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
std::reverse_iterator<const T1 *>
f81(const std::basic_string_view<T1, T2> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2>
std::reverse_iterator<const T1 *>
f82(const std::basic_string_view<T1, T2> &a0) {
  return a0.crend();
}

template <typename T1, typename T2>
std::size_t f83(const std::basic_string_view<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
std::size_t f84(const std::basic_string_view<T1, T2> &a0) {
  return a0.length();
}

template <typename T1, typename T2>
std::size_t f85(const std::basic_string_view<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
bool f86(const std::basic_string_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
const T1 &f87(const std::basic_string_view<T1, T2> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
const T1 &f88(const std::basic_string_view<T1, T2> &a0, std::size_t a1) {
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
void f92(std::basic_string_view<T1, T2> &a0, std::size_t a1) {
  return a0.remove_prefix(a1);
}

template <typename T1, typename T2>
void f93(std::basic_string_view<T1, T2> &a0, std::size_t a1) {
  return a0.remove_suffix(a1);
}

template <typename T1, typename T2>
void f94(std::basic_string_view<T1, T2> &a0,
         std::basic_string_view<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2>
std::size_t f95(const std::basic_string_view<T1, T2> &a0, T1 *a1,
                std::size_t a2, std::size_t a3) {
  return a0.copy(a1, a2, a3);
}

template <typename T1, typename T2>
std::basic_string_view<T1, T2> f96(const std::basic_string_view<T1, T2> &a0,
                                   std::size_t a1, std::size_t a2) {
  return a0.substr(a1, a2);
}

template <typename T1, typename T2>
int f97(const std::basic_string_view<T1, T2> &a0,
        std::basic_string_view<T1, T2> a1) {
  return a0.compare(a1);
}

template <typename T1, typename T2>
int f98(const std::basic_string_view<T1, T2> &a0, std::size_t a1,
        std::size_t a2, std::basic_string_view<T1, T2> a3) {
  return a0.compare(a1, a2, a3);
}

template <typename T1, typename T2>
int f99(const std::basic_string_view<T1, T2> &a0, std::size_t a1,
        std::size_t a2, std::basic_string_view<T1, T2> a3, std::size_t a4,
        std::size_t a5) {
  return a0.compare(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
int f100(const std::basic_string_view<T1, T2> &a0, const T1 *a1) {
  return a0.compare(a1);
}

template <typename T1, typename T2>
int f101(const std::basic_string_view<T1, T2> &a0, std::size_t a1,
         std::size_t a2, const T1 *a3) {
  return a0.compare(a1, a2, a3);
}

template <typename T1, typename T2>
int f102(const std::basic_string_view<T1, T2> &a0, std::size_t a1,
         std::size_t a2, const T1 *a3, std::size_t a4) {
  return a0.compare(a1, a2, a3, a4);
}

template <typename T1, typename T2>
std::size_t f103(const std::basic_string_view<T1, T2> &a0,
                 std::basic_string_view<T1, T2> a1, std::size_t a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2>
std::size_t f104(const std::basic_string_view<T1, T2> &a0, T1 a1,
                 std::size_t a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2>
std::size_t f105(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                 std::size_t a2, std::size_t a3) {
  return a0.find(a1, a2, a3);
}

template <typename T1, typename T2>
std::size_t f106(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                 std::size_t a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2>
std::size_t f107(const std::basic_string_view<T1, T2> &a0,
                 std::basic_string_view<T1, T2> a1, std::size_t a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2>
std::size_t f108(const std::basic_string_view<T1, T2> &a0, T1 a1,
                 std::size_t a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2>
std::size_t f109(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                 std::size_t a2, std::size_t a3) {
  return a0.rfind(a1, a2, a3);
}

template <typename T1, typename T2>
std::size_t f110(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                 std::size_t a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2>
std::size_t f111(const std::basic_string_view<T1, T2> &a0,
                 std::basic_string_view<T1, T2> a1, std::size_t a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2>
std::size_t f112(const std::basic_string_view<T1, T2> &a0, T1 a1,
                 std::size_t a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2>
std::size_t f113(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                 std::size_t a2, std::size_t a3) {
  return a0.find_first_of(a1, a2, a3);
}

template <typename T1, typename T2>
std::size_t f114(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                 std::size_t a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2>
std::size_t f115(const std::basic_string_view<T1, T2> &a0,
                 std::basic_string_view<T1, T2> a1, std::size_t a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2>
std::size_t f116(const std::basic_string_view<T1, T2> &a0, T1 a1,
                 std::size_t a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2>
std::size_t f117(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                 std::size_t a2, std::size_t a3) {
  return a0.find_last_of(a1, a2, a3);
}

template <typename T1, typename T2>
std::size_t f118(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                 std::size_t a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2>
std::size_t f119(const std::basic_string_view<T1, T2> &a0,
                 std::basic_string_view<T1, T2> a1, std::size_t a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2>
std::size_t f120(const std::basic_string_view<T1, T2> &a0, T1 a1,
                 std::size_t a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2>
std::size_t f121(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                 std::size_t a2, std::size_t a3) {
  return a0.find_first_not_of(a1, a2, a3);
}

template <typename T1, typename T2>
std::size_t f122(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                 std::size_t a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2>
std::size_t f123(const std::basic_string_view<T1, T2> &a0,
                 std::basic_string_view<T1, T2> a1, std::size_t a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2>
std::size_t f124(const std::basic_string_view<T1, T2> &a0, T1 a1,
                 std::size_t a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2>
std::size_t f125(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                 std::size_t a2, std::size_t a3) {
  return a0.find_last_not_of(a1, a2, a3);
}

template <typename T1, typename T2>
std::size_t f126(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                 std::size_t a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2> std::size_t f127() {
  return std::basic_string_view<T1, T2>::npos;
}
