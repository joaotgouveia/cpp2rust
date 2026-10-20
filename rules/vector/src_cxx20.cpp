#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <utility>
#include <vector>

template <typename T1, typename T2, typename T3>
typename std::vector<T1, T2>::size_type f187(std::vector<T1, T2> &a0,
                                             const T3 &a1) {
  return std::erase<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::vector<T1, T2>::size_type f188(std::vector<T1, T2> &a0, T3 a1) {
  return std::erase_if<T1, T2, T3>(a0, a1);
}

template <typename T1>
using t55 =
    typename std::vector<bool, T1>::const_reverse_iterator::iterator_concept;

template <typename T1>
using t56 =
    typename std::vector<bool, T1>::const_reverse_iterator::iterator_category;

template <typename T1>
using t57 = typename std::vector<bool, T1>::const_reverse_iterator::value_type;

template <typename T1>
using t58 = typename std::vector<bool, T1>::reverse_iterator::iterator_concept;

template <typename T1>
using t59 = typename std::vector<bool, T1>::reverse_iterator::iterator_category;

template <typename T1>
using t60 = typename std::vector<bool, T1>::reverse_iterator::value_type;

template <typename T1, typename T2>
using t61 = typename std::vector<T1, T2>::iterator::iterator_concept;

template <typename T1, typename T2>
using t62 = typename std::vector<T1, T2>::const_iterator::iterator_concept;

template <typename T1, typename T2>
using t63 =
    typename std::vector<T1, T2>::const_reverse_iterator::iterator_concept;

template <typename T1, typename T2>
using t64 =
    typename std::vector<T1, T2>::const_reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t65 = typename std::vector<T1, T2>::const_reverse_iterator::value_type;

template <typename T1, typename T2>
using t66 = typename std::vector<T1, T2>::reverse_iterator::iterator_concept;

template <typename T1, typename T2>
using t67 = typename std::vector<T1, T2>::reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t68 = typename std::vector<T1, T2>::reverse_iterator::value_type;
