#include "../common/init.h"
#include "../common/traits.h"
#include <deque>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2>
using t29 =
    typename std::deque<T1, T2>::const_reverse_iterator::iterator_concept;

template <typename T1, typename T2>
using t30 =
    typename std::deque<T1, T2>::const_reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t31 = typename std::deque<T1, T2>::const_reverse_iterator::value_type;

template <typename T1, typename T2>
using t32 = typename std::deque<T1, T2>::reverse_iterator::iterator_concept;

template <typename T1, typename T2>
using t33 = typename std::deque<T1, T2>::reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t34 = typename std::deque<T1, T2>::reverse_iterator::value_type;

template <typename T1, typename T2, typename T3>
typename std::deque<T1, T2>::size_type f64(std::deque<T1, T2> &a0,
                                           const T3 &a1) {
  return std::erase<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::deque<T1, T2>::size_type f65(std::deque<T1, T2> &a0, T3 a1) {
  return std::erase_if<T1, T2, T3>(a0, a1);
}
