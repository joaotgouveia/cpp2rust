#include <algorithm>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
using t7 = typename std::reverse_iterator<T1>::iterator_category;

template <typename T1>
using t8 = typename std::reverse_iterator<T1>::value_type;
