#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1>
using t3 = typename std::front_insert_iterator<T1>::difference_type;
