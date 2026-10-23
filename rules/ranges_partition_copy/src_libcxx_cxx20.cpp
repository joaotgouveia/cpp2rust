#include <algorithm>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
using t1 = std::ranges::partition_copy_result<T1, T2, T3>;
