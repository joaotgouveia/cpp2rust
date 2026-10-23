#include <algorithm>
#include <initializer_list>
#include <utility>
#include <vector>

template <typename T1>
using t37 =
    typename std::vector<bool, T1>::const_reverse_iterator::iterator_concept;

template <typename T1>
using t38 = typename std::vector<bool, T1>::reverse_iterator::iterator_concept;

template <typename T1, typename T2>
using t39 = typename std::vector<T1, T2>::iterator::iterator_concept;

template <typename T1, typename T2>
using t40 = typename std::vector<T1, T2>::const_iterator::iterator_concept;

template <typename T1, typename T2>
using t41 =
    typename std::vector<T1, T2>::const_reverse_iterator::iterator_concept;

template <typename T1, typename T2>
using t42 = typename std::vector<T1, T2>::reverse_iterator::iterator_concept;
