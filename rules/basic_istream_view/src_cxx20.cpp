#include <algorithm>
#include <istream>
#include <iterator>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::ranges::basic_istream_view<T1, T2, T3>;
