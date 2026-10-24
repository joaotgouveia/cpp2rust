#include <algorithm>
#include <bitset>
#include <initializer_list>
#include <iterator>
#include <utility>
#include <vector>

template <typename T1, typename T2>
using t37 = typename std::vector<T1, T2>::iterator::iterator_concept;

template <typename T1, typename T2>
using t38 = typename std::vector<T1, T2>::const_iterator::iterator_concept;
