#include <algorithm>
#include <array>
#include <tuple>
#include <utility>

template <std::size_t T1, typename T2, typename T3>
using t23 = std::tuple_element<T1, std::pair<T2, T3>>;
