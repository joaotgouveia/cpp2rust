#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <chrono>
#include <type_traits>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
using t3 = std::common_type<std::pair<T1, T2>, std::pair<T3, T4>>;

template <typename T1, typename T2, typename T3, typename T4>
using t4 =
    typename std::common_type<std::pair<T1, T2>, std::pair<T3, T4>>::type;

template <typename T1, typename T2, typename T3, typename T4>
using t5 = typename std::common_type<std::pair<T1, T2>,
                                     std::pair<T3, T4>>::type::first_type;

template <typename T1, typename T2, typename T3, typename T4>
using t6 = typename std::common_type<std::pair<T1, T2>,
                                     std::pair<T3, T4>>::type::second_type;
