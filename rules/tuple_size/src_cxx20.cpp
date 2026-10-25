#include <algorithm>
#include <array>
#include <cstddef>
#include <ranges>
#include <tuple>
#include <type_traits>
#include <utility>

template <typename T1, typename T2, std::ranges::subrange_kind T3>
using t4 = std::tuple_size<std::ranges::subrange<T1, T2, T3>>;
