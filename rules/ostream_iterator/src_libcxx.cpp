#include <iterator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
using t12 = typename std::ostream_iterator<T1, T2, T3>::iterator_category;

template <typename T1, typename T2, typename T3>
using t13 = typename std::ostream_iterator<T1, T2, T3>::value_type;

template <typename T1, typename T2, typename T3>
using t14 = typename std::ostream_iterator<T1, T2, T3>::difference_type;

template <typename T1, typename T2, typename T3>
using t15 = typename std::ostream_iterator<T1, T2, T3>::pointer;

template <typename T1, typename T2, typename T3>
using t16 = typename std::ostream_iterator<T1, T2, T3>::reference;
