#include <regex>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2> using t1 = std::match_results<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::match_results<T1, T2>::value_type;

template <typename T1, typename T2>
using t3 = typename std::match_results<T1, T2>::const_reference;

template <typename T1, typename T2>
using t4 = typename std::match_results<T1, T2>::reference;

template <typename T1, typename T2>
using t5 = typename std::match_results<T1, T2>::difference_type;

template <typename T1, typename T2>
using t6 = typename std::match_results<T1, T2>::size_type;

template <typename T1, typename T2>
using t7 = typename std::match_results<T1, T2>::allocator_type;

template <typename T1, typename T2>
using t8 = typename std::match_results<T1, T2>::char_type;

template <typename T1, typename T2>
using t9 = typename std::match_results<T1, T2>::string_type;

template <typename T1, typename T2> std::match_results<T1, T2> f1() {
  return std::match_results<T1, T2>();
}

template <typename T1, typename T2>
bool f2(const std::match_results<T1, T2> &a0) {
  return a0.ready();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::size_type
f3(const std::match_results<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::size_type
f4(const std::match_results<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
bool f5(const std::match_results<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::difference_type
f6(const std::match_results<T1, T2> &a0,
   typename std::match_results<T1, T2>::size_type a1) {
  return a0.length(a1);
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::difference_type
f7(const std::match_results<T1, T2> &a0,
   typename std::match_results<T1, T2>::size_type a1) {
  return a0.position(a1);
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::string_type
f8(const std::match_results<T1, T2> &a0,
   typename std::match_results<T1, T2>::size_type a1) {
  return a0.str(a1);
}

template <typename T1, typename T2>
const typename std::match_results<T1, T2>::value_type &
f9(const std::match_results<T1, T2> &a0,
   typename std::match_results<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
const typename std::match_results<T1, T2>::value_type &
f10(const std::match_results<T1, T2> &a0) {
  return a0.prefix();
}

template <typename T1, typename T2>
const typename std::match_results<T1, T2>::value_type &
f11(const std::match_results<T1, T2> &a0) {
  return a0.suffix();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator
f12(const std::match_results<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator
f13(const std::match_results<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator
f14(const std::match_results<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator
f15(const std::match_results<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::string_type
f16(const std::match_results<T1, T2> &a0,
    const typename std::match_results<T1, T2>::char_type *a1,
    std::regex_constants::match_flag_type a2) {
  return a0.format(a1, a2);
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::allocator_type
f17(const std::match_results<T1, T2> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2>
void f18(std::match_results<T1, T2> &a0, std::match_results<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
T3 f19(const std::match_results<T1, T2> &a0, T3 a1,
       const typename std::match_results<T1, T2>::char_type *a2,
       const typename std::match_results<T1, T2>::char_type *a3,
       std::regex_constants::match_flag_type a4) {
  return a0.template format<T3>(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T3 f20(const std::match_results<T1, T2> &a0, T3 a1,
       const std::basic_string<typename std::match_results<T1, T2>::char_type,
                               T4, T5> &a2,
       std::regex_constants::match_flag_type a3) {
  return a0.template format<T3, T4, T5>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<typename std::match_results<T1, T2>::char_type, T3, T4>
f21(const std::match_results<T1, T2> &a0,
    const std::basic_string<typename std::match_results<T1, T2>::char_type, T3,
                            T4> &a1,
    std::regex_constants::match_flag_type a2) {
  return a0.template format<T3, T4>(a1, a2);
}

template <typename T1, typename T2>
using t10 = typename std::match_results<T1, T2>::value_type::value_type;

template <typename T1, typename T2>
using t11 = typename std::match_results<T1, T2>::value_type::difference_type;

template <typename T1, typename T2>
using t12 = typename std::match_results<T1, T2>::value_type::iterator;

template <typename T1, typename T2>
using t13 = typename std::match_results<T1, T2>::value_type::string_type;

template <typename T1, typename T2>
using t14 =
    typename std::match_results<T1, T2>::const_iterator::iterator_category;

template <typename T1, typename T2>
using t15 = typename std::match_results<T1, T2>::const_iterator::value_type;

template <typename T1, typename T2>
using t16 =
    typename std::match_results<T1, T2>::const_iterator::difference_type;

template <typename T1, typename T2>
using t17 = typename std::match_results<T1, T2>::const_iterator::reference;

template <typename T1, typename T2>
using t18 = typename std::match_results<T1, T2>::const_iterator::pointer;

template <typename T1, typename T2>
using t19 = typename std::match_results<T1, T2>::iterator::iterator_category;

template <typename T1, typename T2>
using t20 = typename std::match_results<T1, T2>::iterator::value_type;

template <typename T1, typename T2>
using t21 = typename std::match_results<T1, T2>::iterator::difference_type;

template <typename T1, typename T2>
using t22 = typename std::match_results<T1, T2>::iterator::reference;

template <typename T1, typename T2>
using t23 = typename std::match_results<T1, T2>::iterator::pointer;

template <typename T1, typename T2>
using t24 = typename std::match_results<T1, T2>::string_type::traits_type;

template <typename T1, typename T2>
using t25 = typename std::match_results<T1, T2>::string_type::value_type;

template <typename T1, typename T2>
using t26 = typename std::match_results<T1, T2>::string_type::allocator_type;

template <typename T1, typename T2>
using t27 = typename std::match_results<T1, T2>::string_type::size_type;

template <typename T1, typename T2>
using t28 = typename std::match_results<T1, T2>::string_type::difference_type;

template <typename T1, typename T2>
using t29 = typename std::match_results<T1, T2>::string_type::reference;

template <typename T1, typename T2>
using t30 = typename std::match_results<T1, T2>::string_type::const_reference;

template <typename T1, typename T2>
using t31 = typename std::match_results<T1, T2>::string_type::pointer;

template <typename T1, typename T2>
using t32 = typename std::match_results<T1, T2>::string_type::const_pointer;
