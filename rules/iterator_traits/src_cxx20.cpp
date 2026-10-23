#include <algorithm>
#include <iterator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
using t8 = typename std::iterator_traits<T1 *>::iterator_concept;

template <typename T1>
using t9 = std::iterator_traits<std::counted_iterator<T1>>;

template <typename T1>
using t10 = typename std::iterator_traits<std::counted_iterator<T1>>::pointer;

template <typename T1, typename T2>
using t11 = std::iterator_traits<std::common_iterator<T1, T2>>;

template <typename T1, typename T2>
using t12 = typename std::iterator_traits<
    std::common_iterator<T1, T2>>::iterator_concept;

template <typename T1, typename T2>
using t13 = typename std::iterator_traits<
    std::common_iterator<T1, T2>>::iterator_category;

template <typename T1, typename T2>
using t14 =
    typename std::iterator_traits<std::common_iterator<T1, T2>>::value_type;

template <typename T1, typename T2>
using t15 = typename std::iterator_traits<
    std::common_iterator<T1, T2>>::difference_type;

template <typename T1, typename T2>
using t16 =
    typename std::iterator_traits<std::common_iterator<T1, T2>>::pointer;

template <typename T1, typename T2>
using t17 =
    typename std::iterator_traits<std::common_iterator<T1, T2>>::reference;
