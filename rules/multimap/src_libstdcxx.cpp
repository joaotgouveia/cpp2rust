#include <algorithm>
#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::multimap<T1, T2, T3, T4> f65(T5 a0, T5 a1) {
  return std::multimap<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
using t14 = typename std::multimap<T1, T2, T3, T4>::iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t15 = typename std::multimap<T1, T2, T3, T4>::iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t16 = typename std::multimap<T1, T2, T3, T4>::iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t17 = typename std::multimap<T1, T2, T3, T4>::iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t18 = typename std::multimap<T1, T2, T3, T4>::iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t19 = typename std::multimap<T1, T2, T3, T4>::const_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t20 = typename std::multimap<T1, T2, T3, T4>::const_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t21 = typename std::multimap<T1, T2, T3, T4>::const_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t22 =
    typename std::multimap<T1, T2, T3, T4>::const_iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t23 =
    typename std::multimap<T1, T2, T3, T4>::const_iterator::difference_type;
