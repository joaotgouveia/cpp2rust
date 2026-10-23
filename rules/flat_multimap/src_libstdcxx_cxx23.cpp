#include <flat_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t14 = typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t15 = typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t16 = typename std::flat_multimap<T1, T2, T3, T4, T5>::reverse_iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t17 =
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_reverse_iterator;
