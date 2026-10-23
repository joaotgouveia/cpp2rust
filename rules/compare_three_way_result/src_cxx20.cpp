#include <compare>
#include <utility>

template <typename T1, typename T2>
using t1 = std::compare_three_way_result<T1, T2>;
