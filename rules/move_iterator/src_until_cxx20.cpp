#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1>
using t7 = typename std::move_iterator<T1>::iterator_category;
