#include "../common/init.h"
#include "../common/traits.h"
#include <initializer_list>
#include <list>
#include <utility>

template <typename T1, typename T2, typename T3>
typename std::list<T1, T2>::size_type f76(std::list<T1, T2> &a0, const T3 &a1) {
  return std::erase<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::list<T1, T2>::size_type f77(std::list<T1, T2> &a0, T3 a1) {
  return std::erase_if<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2>
typename std::list<T1, T2>::size_type f78(std::list<T1, T2> &a0, const T1 &a1) {
  return a0.remove(a1);
}

template <typename T1, typename T2>
typename std::list<T1, T2>::size_type f79(std::list<T1, T2> &a0) {
  return a0.unique();
}

template <typename T1, typename T2, typename T3>
typename std::list<T1, T2>::size_type f80(std::list<T1, T2> &a0, T3 a1) {
  return a0.template remove_if<T3>(a1);
}

template <typename T1, typename T2, typename T3>
typename std::list<T1, T2>::size_type f81(std::list<T1, T2> &a0, T3 a1) {
  return a0.template unique<T3>(a1);
}

template <typename T1, typename T2>
using t17 =
    typename std::list<T1, T2>::const_reverse_iterator::iterator_concept;

template <typename T1, typename T2>
using t18 =
    typename std::list<T1, T2>::const_reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t19 = typename std::list<T1, T2>::const_reverse_iterator::value_type;

template <typename T1, typename T2>
using t20 = typename std::list<T1, T2>::reverse_iterator::iterator_concept;

template <typename T1, typename T2>
using t21 = typename std::list<T1, T2>::reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t22 = typename std::list<T1, T2>::reverse_iterator::value_type;
