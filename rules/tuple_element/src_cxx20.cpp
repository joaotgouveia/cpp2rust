#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <array>
#include <ranges>
#include <tuple>
#include <utility>

template <typename T1, typename T2, std::ranges::subrange_kind T3>
using t15 = std::tuple_element<0, const std::ranges::subrange<T1, T2, T3>>;

template <typename T1, typename T2, std::ranges::subrange_kind T3>
using t16 =
    typename std::tuple_element<0,
                                const std::ranges::subrange<T1, T2, T3>>::type;

template <typename T1, typename T2, std::ranges::subrange_kind T3>
using t17 = std::tuple_element<0, std::ranges::subrange<T1, T2, T3>>;

template <typename T1, typename T2, std::ranges::subrange_kind T3>
using t18 =
    typename std::tuple_element<0, std::ranges::subrange<T1, T2, T3>>::type;

template <typename T1, typename T2, std::ranges::subrange_kind T3>
using t19 = std::tuple_element<1, const std::ranges::subrange<T1, T2, T3>>;

template <typename T1, typename T2, std::ranges::subrange_kind T3>
using t20 =
    typename std::tuple_element<1,
                                const std::ranges::subrange<T1, T2, T3>>::type;

template <typename T1, typename T2, std::ranges::subrange_kind T3>
using t21 = std::tuple_element<1, std::ranges::subrange<T1, T2, T3>>;

template <typename T1, typename T2, std::ranges::subrange_kind T3>
using t22 =
    typename std::tuple_element<1, std::ranges::subrange<T1, T2, T3>>::type;
