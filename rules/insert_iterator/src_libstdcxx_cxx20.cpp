#include <algorithm>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
using t3 = typename std::insert_iterator<T1>::difference_type;
