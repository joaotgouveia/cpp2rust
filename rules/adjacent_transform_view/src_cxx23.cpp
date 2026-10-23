#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2, std::size_t T3>
using t1 = std::ranges::adjacent_transform_view<T1, T2, T3>;
