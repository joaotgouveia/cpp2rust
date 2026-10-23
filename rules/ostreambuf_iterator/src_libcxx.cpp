#include <iterator>
#include <ostream>
#include <streambuf>
#include <utility>

template <typename T1, typename T2>
using t7 = typename std::ostreambuf_iterator<T1, T2>::iterator_category;

template <typename T1, typename T2>
using t8 = typename std::ostreambuf_iterator<T1, T2>::value_type;

template <typename T1, typename T2>
using t9 = typename std::ostreambuf_iterator<T1, T2>::difference_type;

template <typename T1, typename T2>
using t10 = typename std::ostreambuf_iterator<T1, T2>::pointer;

template <typename T1, typename T2>
using t11 = typename std::ostreambuf_iterator<T1, T2>::reference;
