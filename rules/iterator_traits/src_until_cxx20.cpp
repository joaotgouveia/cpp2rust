#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1> using t8 = std::iterator_traits<const T1 *>;

template <typename T1>
using t9 = typename std::iterator_traits<const T1 *>::iterator_category;

template <typename T1>
using t10 = typename std::iterator_traits<const T1 *>::value_type;

template <typename T1>
using t11 = typename std::iterator_traits<const T1 *>::difference_type;

template <typename T1>
using t12 = typename std::iterator_traits<const T1 *>::pointer;

template <typename T1>
using t13 = typename std::iterator_traits<const T1 *>::reference;
