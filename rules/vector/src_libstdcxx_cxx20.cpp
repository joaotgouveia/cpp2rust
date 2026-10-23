#include <algorithm>
#include <initializer_list>
#include <utility>
#include <vector>

template <typename T1, typename T2, typename T3>
typename std::vector<T1, T2>::size_type f193(std::vector<T1, T2> &a0,
                                             const T3 &a1) {
  return std::erase<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::vector<T1, T2>::size_type f194(std::vector<T1, T2> &a0, T3 a1) {
  return std::erase_if<T1, T2, T3>(a0, a1);
}

template <typename T1>
using t61 =
    typename std::vector<bool, T1>::const_reverse_iterator::iterator_category;

template <typename T1>
using t62 = typename std::vector<bool, T1>::const_reverse_iterator::value_type;

template <typename T1>
using t63 = typename std::vector<bool, T1>::reverse_iterator::iterator_category;

template <typename T1>
using t64 = typename std::vector<bool, T1>::reverse_iterator::value_type;

template <typename T1, typename T2>
using t65 =
    typename std::vector<T1, T2>::const_reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t66 = typename std::vector<T1, T2>::const_reverse_iterator::value_type;

template <typename T1, typename T2>
using t67 = typename std::vector<T1, T2>::reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t68 = typename std::vector<T1, T2>::reverse_iterator::value_type;
