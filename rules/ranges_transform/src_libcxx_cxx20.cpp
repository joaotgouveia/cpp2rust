#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::ranges::binary_transform_result<T1, T2, T3>;

template <typename T1, typename T2>
using t2 = std::ranges::unary_transform_result<T1, T2>;
