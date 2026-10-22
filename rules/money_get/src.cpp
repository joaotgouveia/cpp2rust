#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <cstddef>
#include <iomanip>
#include <ios>
#include <locale>
#include <utility>

template <typename T1, typename T2> using t1 = std::money_get<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::money_get<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::money_get<T1, T2>::iter_type;

template <typename T1, typename T2>
using t4 = typename std::money_get<T1, T2>::string_type;

template <typename T1, typename T2> std::money_get<T1, T2> f1(std::size_t a0) {
  return std::money_get<T1, T2>(a0);
}

template <typename T1, typename T2>
typename std::money_get<T1, T2>::iter_type
f2(const std::money_get<T1, T2> &a0,
   typename std::money_get<T1, T2>::iter_type a1,
   typename std::money_get<T1, T2>::iter_type a2, bool a3, std::ios_base &a4,
   std::ios_base::iostate &a5, long double &a6) {
  return a0.get(a1, a2, a3, a4, a5, a6);
}

template <typename T1, typename T2>
typename std::money_get<T1, T2>::iter_type
f3(const std::money_get<T1, T2> &a0,
   typename std::money_get<T1, T2>::iter_type a1,
   typename std::money_get<T1, T2>::iter_type a2, bool a3, std::ios_base &a4,
   std::ios_base::iostate &a5,
   typename std::money_get<T1, T2>::string_type &a6) {
  return a0.get(a1, a2, a3, a4, a5, a6);
}

template <typename T1, typename T2>
using t5 = typename std::money_get<T1, T2>::string_type::traits_type;

template <typename T1, typename T2>
using t6 = typename std::money_get<T1, T2>::string_type::value_type;

template <typename T1, typename T2>
using t7 = typename std::money_get<T1, T2>::string_type::allocator_type;

template <typename T1, typename T2>
using t8 = typename std::money_get<T1, T2>::string_type::size_type;

template <typename T1, typename T2>
using t9 = typename std::money_get<T1, T2>::string_type::difference_type;

template <typename T1, typename T2>
using t10 = typename std::money_get<T1, T2>::string_type::reference;

template <typename T1, typename T2>
using t11 = typename std::money_get<T1, T2>::string_type::const_reference;

template <typename T1, typename T2>
using t12 = typename std::money_get<T1, T2>::string_type::pointer;

template <typename T1, typename T2>
using t13 = typename std::money_get<T1, T2>::string_type::const_pointer;

template <typename T1, typename T2> const std::locale::id &f4() {
  return std::money_get<T1, T2>::id;
}
