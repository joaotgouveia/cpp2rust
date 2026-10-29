#include <algorithm>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t1 = std::iterator<T1, T2, T3, T4, T5>;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t2 = typename std::iterator<T1, T2, T3, T4, T5>::iterator_category;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t3 = typename std::iterator<T1, T2, T3, T4, T5>::value_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t4 = typename std::iterator<T1, T2, T3, T4, T5>::difference_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t5 = typename std::iterator<T1, T2, T3, T4, T5>::pointer;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t6 = typename std::iterator<T1, T2, T3, T4, T5>::reference;
