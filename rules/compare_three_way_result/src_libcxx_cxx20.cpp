#include <compare>
#include <utility>

template <typename T1, typename T2>
using t2 = std::compare_three_way_result_t<T1, T2>;
