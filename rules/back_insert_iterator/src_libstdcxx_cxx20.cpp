#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1>
using t3 = typename std::back_insert_iterator<T1>::difference_type;
