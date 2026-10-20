#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1>
using t14 = typename std::iterator_traits<T1 *>::iterator_concept;

template <typename T1>
using t15 = std::iterator_traits<std::counted_iterator<T1>>;

template <typename T1>
using t16 = typename std::iterator_traits<std::counted_iterator<T1>>::pointer;

template <typename T1, typename T2>
using t17 = std::iterator_traits<std::common_iterator<T1, T2>>;

template <typename T1, typename T2>
using t18 = typename std::iterator_traits<
    std::common_iterator<T1, T2>>::iterator_concept;

template <typename T1, typename T2>
using t19 = typename std::iterator_traits<
    std::common_iterator<T1, T2>>::iterator_category;

template <typename T1, typename T2>
using t20 =
    typename std::iterator_traits<std::common_iterator<T1, T2>>::value_type;

template <typename T1, typename T2>
using t21 = typename std::iterator_traits<
    std::common_iterator<T1, T2>>::difference_type;

template <typename T1, typename T2>
using t22 =
    typename std::iterator_traits<std::common_iterator<T1, T2>>::pointer;

template <typename T1, typename T2>
using t23 =
    typename std::iterator_traits<std::common_iterator<T1, T2>>::reference;
