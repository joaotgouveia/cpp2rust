#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <array>
#include <tuple>
#include <utility>

template <typename T1, typename T2, std::ranges::subrange_kind T3>
using t7 = std::tuple_size<std::ranges::subrange<T1, T2, T3>>;
