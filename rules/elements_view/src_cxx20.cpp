#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, std::size_t T2>
using t1 = std::ranges::elements_view<T1, T2>;
