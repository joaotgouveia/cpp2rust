#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <cstddef>
#include <iomanip>
#include <ios>
#include <locale>
#include <string>
#include <utility>

template <typename T1, bool T2> using t1 = std::moneypunct<T1, T2>;

template <typename T1, bool T2>
using t2 = typename std::moneypunct<T1, T2>::char_type;

template <typename T1, bool T2>
using t3 = typename std::moneypunct<T1, T2>::string_type;

template <typename T1, bool T2> std::moneypunct<T1, T2> f1(std::size_t a0) {
  return std::moneypunct<T1, T2>(a0);
}

template <typename T1, bool T2>
typename std::moneypunct<T1, T2>::char_type
f2(const std::moneypunct<T1, T2> &a0) {
  return a0.decimal_point();
}

template <typename T1, bool T2>
typename std::moneypunct<T1, T2>::char_type
f3(const std::moneypunct<T1, T2> &a0) {
  return a0.thousands_sep();
}

template <typename T1, bool T2>
std::string f4(const std::moneypunct<T1, T2> &a0) {
  return a0.grouping();
}

template <typename T1, bool T2>
typename std::moneypunct<T1, T2>::string_type
f5(const std::moneypunct<T1, T2> &a0) {
  return a0.curr_symbol();
}

template <typename T1, bool T2>
typename std::moneypunct<T1, T2>::string_type
f6(const std::moneypunct<T1, T2> &a0) {
  return a0.positive_sign();
}

template <typename T1, bool T2>
typename std::moneypunct<T1, T2>::string_type
f7(const std::moneypunct<T1, T2> &a0) {
  return a0.negative_sign();
}

template <typename T1, bool T2> int f8(const std::moneypunct<T1, T2> &a0) {
  return a0.frac_digits();
}

template <typename T1, bool T2>
typename std::moneypunct<T1, T2>::pattern
f9(const std::moneypunct<T1, T2> &a0) {
  return a0.pos_format();
}

template <typename T1, bool T2>
typename std::moneypunct<T1, T2>::pattern
f10(const std::moneypunct<T1, T2> &a0) {
  return a0.neg_format();
}

template <typename T1, bool T2>
using t4 = typename std::moneypunct<T1, T2>::string_type::traits_type;

template <typename T1, bool T2>
using t5 = typename std::moneypunct<T1, T2>::string_type::value_type;

template <typename T1, bool T2>
using t6 = typename std::moneypunct<T1, T2>::string_type::allocator_type;

template <typename T1, bool T2>
using t7 = typename std::moneypunct<T1, T2>::string_type::size_type;

template <typename T1, bool T2>
using t8 = typename std::moneypunct<T1, T2>::string_type::difference_type;

template <typename T1, bool T2>
using t9 = typename std::moneypunct<T1, T2>::string_type::reference;

template <typename T1, bool T2>
using t10 = typename std::moneypunct<T1, T2>::string_type::const_reference;

template <typename T1, bool T2>
using t11 = typename std::moneypunct<T1, T2>::string_type::pointer;

template <typename T1, bool T2>
using t12 = typename std::moneypunct<T1, T2>::string_type::const_pointer;

template <typename T1, bool T2> bool f11() {
  return std::moneypunct<T1, T2>::intl;
}

template <typename T1, bool T2> const std::locale::id &f12() {
  return std::moneypunct<T1, T2>::id;
}
