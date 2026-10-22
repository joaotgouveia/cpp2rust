#include "../common/init.h"
#include "../common/traits.h"
#include <sstream>
#include <utility>

template <typename T1, typename T2, typename T3>
using t8 = typename std::basic_stringbuf<T1, T2, T3>::string_type;

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3>
f36(const typename std::basic_stringbuf<T1, T2, T3>::string_type &a0,
    std::ios_base::openmode a1) {
  return std::basic_stringbuf<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringbuf<T1, T2, T3>::string_type
f37(const std::basic_stringbuf<T1, T2, T3> &a0) {
  return a0.str();
}

template <typename T1, typename T2, typename T3>
void f38(std::basic_stringbuf<T1, T2, T3> &a0,
         const typename std::basic_stringbuf<T1, T2, T3>::string_type &a1) {
  return a0.str(a1);
}

template <typename T1, typename T2, typename T3>
using t9 = typename std::basic_stringbuf<T1, T2, T3>::string_type::traits_type;

template <typename T1, typename T2, typename T3>
using t10 = typename std::basic_stringbuf<T1, T2, T3>::string_type::value_type;

template <typename T1, typename T2, typename T3>
using t11 =
    typename std::basic_stringbuf<T1, T2, T3>::string_type::allocator_type;

template <typename T1, typename T2, typename T3>
using t12 = typename std::basic_stringbuf<T1, T2, T3>::string_type::size_type;

template <typename T1, typename T2, typename T3>
using t13 =
    typename std::basic_stringbuf<T1, T2, T3>::string_type::difference_type;

template <typename T1, typename T2, typename T3>
using t14 = typename std::basic_stringbuf<T1, T2, T3>::string_type::reference;

template <typename T1, typename T2, typename T3>
using t15 =
    typename std::basic_stringbuf<T1, T2, T3>::string_type::const_reference;

template <typename T1, typename T2, typename T3>
using t16 = typename std::basic_stringbuf<T1, T2, T3>::string_type::pointer;

template <typename T1, typename T2, typename T3>
using t17 =
    typename std::basic_stringbuf<T1, T2, T3>::string_type::const_pointer;
