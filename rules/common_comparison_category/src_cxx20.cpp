#include <compare>
#include <utility>

template <typename... T1> using t1 = std::common_comparison_category<T1...>;

template <typename... T1>
using t2 = typename std::common_comparison_category<T1...>::type;
