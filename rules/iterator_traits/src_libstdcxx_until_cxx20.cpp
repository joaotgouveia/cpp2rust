#include <algorithm>
#include <utility>

template <typename T1> using t18 = std::iterator_traits<const T1 *>;

template <typename T1>
using t19 = typename std::iterator_traits<const T1 *>::iterator_category;

template <typename T1>
using t20 = typename std::iterator_traits<const T1 *>::value_type;

template <typename T1>
using t21 = typename std::iterator_traits<const T1 *>::difference_type;

template <typename T1>
using t22 = typename std::iterator_traits<const T1 *>::pointer;

template <typename T1>
using t23 = typename std::iterator_traits<const T1 *>::reference;
