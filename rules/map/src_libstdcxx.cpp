#include <algorithm>
#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::map<T1, T2, T3, T4> f80(T5 a0, T5 a1) {
  return std::map<T1, T2, T3, T4>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
using t22 = typename std::map<T1, T2, T3, T4>::iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t23 = typename std::map<T1, T2, T3, T4>::iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t24 = typename std::map<T1, T2, T3, T4>::iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t25 = typename std::map<T1, T2, T3, T4>::iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t26 = typename std::map<T1, T2, T3, T4>::iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t27 = typename std::map<T1, T2, T3, T4>::const_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t28 = typename std::map<T1, T2, T3, T4>::const_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t29 = typename std::map<T1, T2, T3, T4>::const_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t30 =
    typename std::map<T1, T2, T3, T4>::const_iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t31 = typename std::map<T1, T2, T3, T4>::const_iterator::difference_type;
