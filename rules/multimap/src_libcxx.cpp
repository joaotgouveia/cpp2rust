#include "../common/init.h"
#include "../common/traits.h"
#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4>
f81(const typename std::multimap<T1, T2, T3, T4>::key_compare &a0) {
  return std::multimap<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4>
f82(const typename std::multimap<T1, T2, T3, T4>::key_compare &a0,
    const typename std::multimap<T1, T2, T3, T4>::allocator_type &a1) {
  return std::multimap<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4>
f83(std::initializer_list<typename std::multimap<T1, T2, T3, T4>::value_type>
        a0,
    const typename std::multimap<T1, T2, T3, T4>::key_compare &a1) {
  return std::multimap<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4>
f84(std::initializer_list<typename std::multimap<T1, T2, T3, T4>::value_type>
        a0,
    const typename std::multimap<T1, T2, T3, T4>::key_compare &a1,
    const typename std::multimap<T1, T2, T3, T4>::allocator_type &a2) {
  return std::multimap<T1, T2, T3, T4>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::multimap<T1, T2, T3, T4>
f85(T5 a0, T5 a1,
    const typename std::multimap<T1, T2, T3, T4>::key_compare &a2) {
  return std::multimap<T1, T2, T3, T4>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::multimap<T1, T2, T3, T4>
f86(T5 a0, T5 a1, const typename std::multimap<T1, T2, T3, T4>::key_compare &a2,
    const typename std::multimap<T1, T2, T3, T4>::allocator_type &a3) {
  return std::multimap<T1, T2, T3, T4>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
using t26 = typename std::multimap<T1, T2, T3, T4>::value_type::first_type;

template <typename T1, typename T2, typename T3, typename T4>
using t27 = typename std::multimap<T1, T2, T3, T4>::value_type::second_type;

template <typename T1, typename T2, typename T3, typename T4>
using t28 =
    typename std::multimap<T1, T2, T3, T4>::reverse_iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t29 = typename std::multimap<T1, T2, T3, T4>::reverse_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t30 =
    typename std::multimap<T1, T2, T3, T4>::reverse_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t31 =
    typename std::multimap<T1, T2, T3, T4>::reverse_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t32 = typename std::multimap<T1, T2, T3, T4>::reverse_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t33 =
    typename std::multimap<T1, T2, T3,
                           T4>::const_reverse_iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t34 =
    typename std::multimap<T1, T2, T3, T4>::const_reverse_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t35 =
    typename std::multimap<T1, T2, T3, T4>::const_reverse_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t36 = typename std::multimap<T1, T2, T3,
                                   T4>::const_reverse_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t37 =
    typename std::multimap<T1, T2, T3, T4>::const_reverse_iterator::reference;
