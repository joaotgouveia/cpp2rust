#include <algorithm>
#include <iterator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
using t7 = typename std::move_iterator<T1>::iterator_category;
