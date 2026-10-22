#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::ranges::set_intersection_result<T1, T2, T3>;
