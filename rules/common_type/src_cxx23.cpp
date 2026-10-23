#include <algorithm>
#include <chrono>
#include <tuple>
#include <type_traits>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
using t3 = std::common_type<std::pair<T1, T2>, std::pair<T3, T4>>;

template <typename T1, typename T2, typename T3, typename T4>
using t4 =
    typename std::common_type<std::pair<T1, T2>, std::pair<T3, T4>>::type;
