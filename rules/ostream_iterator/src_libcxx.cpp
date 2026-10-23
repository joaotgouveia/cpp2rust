#include <iterator>
#include <ostream>
#include <utility>

template <typename T1, typename T2, typename T3>
using t6 = typename std::ostream_iterator<T1, T2, T3>::iterator_category;

template <typename T1, typename T2, typename T3>
using t7 = typename std::ostream_iterator<T1, T2, T3>::value_type;

template <typename T1, typename T2, typename T3>
using t8 = typename std::ostream_iterator<T1, T2, T3>::difference_type;

template <typename T1, typename T2, typename T3>
using t9 = typename std::ostream_iterator<T1, T2, T3>::pointer;

template <typename T1, typename T2, typename T3>
using t10 = typename std::ostream_iterator<T1, T2, T3>::reference;
