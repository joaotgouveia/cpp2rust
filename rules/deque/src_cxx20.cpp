#include <deque>
#include <initializer_list>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
using t27 =
    typename std::deque<T1, T2>::const_reverse_iterator::iterator_concept;

template <typename T1, typename T2>
using t28 = typename std::deque<T1, T2>::reverse_iterator::iterator_concept;

template <typename T1, typename T2, typename T3>
typename std::deque<T1, T2>::size_type f49(std::deque<T1, T2> &a0,
                                           const T3 &a1) {
  return std::erase<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::deque<T1, T2>::size_type f50(std::deque<T1, T2> &a0, T3 a1) {
  return std::erase_if<T1, T2, T3>(a0, a1);
}
