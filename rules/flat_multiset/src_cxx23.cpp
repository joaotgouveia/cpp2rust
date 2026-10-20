#include "../common/init.h"
#include "../common/traits.h"
#include <flat_set>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::flat_multiset<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::flat_multiset<T1, T2, T3>::key_type;

template <typename T1, typename T2, typename T3>
using t3 = typename std::flat_multiset<T1, T2, T3>::value_type;

template <typename T1, typename T2, typename T3>
using t4 = typename std::flat_multiset<T1, T2, T3>::key_compare;

template <typename T1, typename T2, typename T3>
using t5 = typename std::flat_multiset<T1, T2, T3>::reference;

template <typename T1, typename T2, typename T3>
using t6 = typename std::flat_multiset<T1, T2, T3>::const_reference;

template <typename T1, typename T2, typename T3>
using t7 = typename std::flat_multiset<T1, T2, T3>::size_type;

template <typename T1, typename T2, typename T3>
using t8 = typename std::flat_multiset<T1, T2, T3>::difference_type;

template <typename T1, typename T2, typename T3>
using t9 = typename std::flat_multiset<T1, T2, T3>::iterator;

template <typename T1, typename T2, typename T3>
using t10 = typename std::flat_multiset<T1, T2, T3>::const_iterator;

template <typename T1, typename T2, typename T3>
using t11 = typename std::flat_multiset<T1, T2, T3>::reverse_iterator;

template <typename T1, typename T2, typename T3>
using t12 = typename std::flat_multiset<T1, T2, T3>::const_reverse_iterator;

template <typename T1, typename T2, typename T3>
using t13 = typename std::flat_multiset<T1, T2, T3>::container_type;

template <typename T1, typename T2, typename T3>
using t14 = typename std::flat_multiset<T1, T2, T3>::value_compare;
