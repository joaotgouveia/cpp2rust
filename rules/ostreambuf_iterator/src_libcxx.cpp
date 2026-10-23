#include <iterator>
#include <utility>

template <typename T1, typename T2>
using t12 = typename std::ostreambuf_iterator<T1, T2>::iterator_category;

template <typename T1, typename T2>
using t13 = typename std::ostreambuf_iterator<T1, T2>::value_type;

template <typename T1, typename T2>
using t14 = typename std::ostreambuf_iterator<T1, T2>::difference_type;

template <typename T1, typename T2>
using t15 = typename std::ostreambuf_iterator<T1, T2>::pointer;

template <typename T1, typename T2>
using t16 = typename std::ostreambuf_iterator<T1, T2>::reference;
