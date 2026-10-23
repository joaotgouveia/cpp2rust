#include <compare>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
using t2 = std::compare_three_way_result_t<T1, T2>;
