#include <algorithm>
#include <cstddef>
#include <iomanip>
#include <locale>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, bool T2> using t1 = std::moneypunct_byname<T1, T2>;

template <typename T1, bool T2>
using t2 = typename std::moneypunct_byname<T1, T2>::char_type;

template <typename T1, bool T2>
using t3 = typename std::moneypunct_byname<T1, T2>::string_type;

template <typename T1, bool T2>
std::moneypunct_byname<T1, T2> f1(const char *a0, std::size_t a1) {
  return std::moneypunct_byname<T1, T2>(a0, a1);
}

template <typename T1, bool T2>
std::moneypunct_byname<T1, T2> f2(const std::string &a0, std::size_t a1) {
  return std::moneypunct_byname<T1, T2>(a0, a1);
}

template <typename T1, bool T2>
typename std::moneypunct_byname<T1, T2>::char_type
f3(const std::moneypunct_byname<T1, T2> &a0) {
  return a0.decimal_point();
}

template <typename T1, bool T2>
typename std::moneypunct_byname<T1, T2>::char_type
f4(const std::moneypunct_byname<T1, T2> &a0) {
  return a0.thousands_sep();
}

template <typename T1, bool T2>
std::string f5(const std::moneypunct_byname<T1, T2> &a0) {
  return a0.grouping();
}

template <typename T1, bool T2>
typename std::moneypunct_byname<T1, T2>::string_type
f6(const std::moneypunct_byname<T1, T2> &a0) {
  return a0.curr_symbol();
}

template <typename T1, bool T2>
typename std::moneypunct_byname<T1, T2>::string_type
f7(const std::moneypunct_byname<T1, T2> &a0) {
  return a0.positive_sign();
}

template <typename T1, bool T2>
typename std::moneypunct_byname<T1, T2>::string_type
f8(const std::moneypunct_byname<T1, T2> &a0) {
  return a0.negative_sign();
}

template <typename T1, bool T2>
int f9(const std::moneypunct_byname<T1, T2> &a0) {
  return a0.frac_digits();
}

template <typename T1, bool T2>
typename std::moneypunct_byname<T1, T2>::pattern
f10(const std::moneypunct_byname<T1, T2> &a0) {
  return a0.pos_format();
}

template <typename T1, bool T2>
typename std::moneypunct_byname<T1, T2>::pattern
f11(const std::moneypunct_byname<T1, T2> &a0) {
  return a0.neg_format();
}

template <typename T1, bool T2>
using t4 = typename std::moneypunct_byname<T1, T2>::string_type::traits_type;

template <typename T1, bool T2>
using t5 = typename std::moneypunct_byname<T1, T2>::string_type::value_type;

template <typename T1, bool T2>
using t6 = typename std::moneypunct_byname<T1, T2>::string_type::allocator_type;

template <typename T1, bool T2>
using t7 = typename std::moneypunct_byname<T1, T2>::string_type::size_type;

template <typename T1, bool T2>
using t8 =
    typename std::moneypunct_byname<T1, T2>::string_type::difference_type;

template <typename T1, bool T2>
using t9 = typename std::moneypunct_byname<T1, T2>::string_type::reference;

template <typename T1, bool T2>
using t10 =
    typename std::moneypunct_byname<T1, T2>::string_type::const_reference;

template <typename T1, bool T2>
using t11 = typename std::moneypunct_byname<T1, T2>::string_type::pointer;

template <typename T1, bool T2>
using t12 = typename std::moneypunct_byname<T1, T2>::string_type::const_pointer;
