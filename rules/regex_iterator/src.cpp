#include <regex>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
using t1 = std::regex_iterator<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::regex_iterator<T1, T2, T3>::regex_type;

template <typename T1, typename T2, typename T3>
using t3 = typename std::regex_iterator<T1, T2, T3>::value_type;

template <typename T1, typename T2, typename T3>
using t4 = typename std::regex_iterator<T1, T2, T3>::difference_type;

template <typename T1, typename T2, typename T3>
using t5 = typename std::regex_iterator<T1, T2, T3>::pointer;

template <typename T1, typename T2, typename T3>
using t6 = typename std::regex_iterator<T1, T2, T3>::reference;

template <typename T1, typename T2, typename T3>
using t7 = typename std::regex_iterator<T1, T2, T3>::iterator_category;

template <typename T1, typename T2, typename T3>
std::regex_iterator<T1, T2, T3> f1() {
  return std::regex_iterator<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::regex_iterator<T1, T2, T3>
f2(T1 a0, T1 a1, const typename std::regex_iterator<T1, T2, T3>::regex_type &a2,
   std::regex_constants::match_flag_type a3) {
  return std::regex_iterator<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
bool f3(const std::regex_iterator<T1, T2, T3> &a0,
        const std::regex_iterator<T1, T2, T3> &a1) {
  return a0.operator==(a1);
}

template <typename T1, typename T2, typename T3>
const typename std::regex_iterator<T1, T2, T3>::value_type &
f4(const std::regex_iterator<T1, T2, T3> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, typename T3>
const typename std::regex_iterator<T1, T2, T3>::value_type *
f5(const std::regex_iterator<T1, T2, T3> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2, typename T3>
std::regex_iterator<T1, T2, T3> &f6(std::regex_iterator<T1, T2, T3> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2, typename T3>
std::regex_iterator<T1, T2, T3> f7(std::regex_iterator<T1, T2, T3> &a0,
                                   int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2, typename T3>
using t8 = typename std::regex_iterator<T1, T2, T3>::regex_type::value_type;

template <typename T1, typename T2, typename T3>
using t9 = typename std::regex_iterator<T1, T2, T3>::regex_type::traits_type;

template <typename T1, typename T2, typename T3>
using t10 = typename std::regex_iterator<T1, T2, T3>::regex_type::string_type;

template <typename T1, typename T2, typename T3>
using t11 = typename std::regex_iterator<T1, T2, T3>::regex_type::flag_type;

template <typename T1, typename T2, typename T3>
using t12 = typename std::regex_iterator<T1, T2, T3>::regex_type::locale_type;

template <typename T1, typename T2, typename T3>
using t13 = typename std::regex_iterator<T1, T2, T3>::value_type::value_type;

template <typename T1, typename T2, typename T3>
using t14 =
    typename std::regex_iterator<T1, T2, T3>::value_type::const_reference;

template <typename T1, typename T2, typename T3>
using t15 = typename std::regex_iterator<T1, T2, T3>::value_type::reference;

template <typename T1, typename T2, typename T3>
using t16 =
    typename std::regex_iterator<T1, T2, T3>::value_type::difference_type;

template <typename T1, typename T2, typename T3>
using t17 = typename std::regex_iterator<T1, T2, T3>::value_type::size_type;

template <typename T1, typename T2, typename T3>
using t18 =
    typename std::regex_iterator<T1, T2, T3>::value_type::allocator_type;

template <typename T1, typename T2, typename T3>
using t19 = typename std::regex_iterator<T1, T2, T3>::value_type::char_type;

template <typename T1, typename T2, typename T3>
using t20 = typename std::regex_iterator<T1, T2, T3>::value_type::string_type;
