#include <algorithm>
#include <initializer_list>
#include <utility>
#include <vector>

template <typename T1, typename T2>
using t35 = typename std::vector<T1, T2>::iterator::iterator_concept;

template <typename T1, typename T2>
using t36 = typename std::vector<T1, T2>::const_iterator::iterator_concept;
