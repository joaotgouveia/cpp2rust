#include "../common/init.h"
#include "../common/traits.h"
#include <deque>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2>
using t31 =
    typename std::deque<T1, T2>::const_reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t32 = typename std::deque<T1, T2>::const_reverse_iterator::value_type;

template <typename T1, typename T2>
using t33 = typename std::deque<T1, T2>::reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t34 = typename std::deque<T1, T2>::reverse_iterator::value_type;
