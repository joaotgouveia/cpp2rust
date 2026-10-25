#include <algorithm>
#include <iterator>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
using t1 = std::ranges::lazy_split_view<T1, T2>;
