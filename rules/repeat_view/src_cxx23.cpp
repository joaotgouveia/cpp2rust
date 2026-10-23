#include <algorithm>
#include <iterator>
#include <ranges>
#include <tuple>
#include <utility>

template <typename T1, typename T2> using t1 = std::ranges::repeat_view<T1, T2>;
