#include <algorithm>
#include <array>
#include <tuple>
#include <utility>

template <std::size_t T1, typename T2>
using t1 = std::tuple_element<T1, const T2>;

template <std::size_t T1, typename T2>
using t2 = typename std::tuple_element<T1, const T2>::type;

template <std::size_t T1, typename T2>
using t3 = std::tuple_element<T1, volatile T2>;

template <std::size_t T1, typename T2>
using t4 = typename std::tuple_element<T1, volatile T2>::type;

template <std::size_t T1, typename T2>
using t5 = std::tuple_element<T1, const volatile T2>;

template <std::size_t T1, typename T2>
using t6 = typename std::tuple_element<T1, const volatile T2>::type;

template <std::size_t T1, typename... T2>
using t7 = std::tuple_element<T1, std::tuple<T2...>>;

template <std::size_t T1, typename... T2>
using t8 = typename std::tuple_element<T1, std::tuple<T2...>>::type;

template <typename T1, typename T2>
using t9 = std::tuple_element<0, std::pair<T1, T2>>;

template <typename T1, typename T2>
using t10 = typename std::tuple_element<0, std::pair<T1, T2>>::type;

template <typename T1, typename T2>
using t11 = std::tuple_element<1, std::pair<T1, T2>>;

template <typename T1, typename T2>
using t12 = typename std::tuple_element<1, std::pair<T1, T2>>::type;

template <std::size_t T1, typename T2, std::size_t T3>
using t13 = std::tuple_element<T1, std::array<T2, T3>>;

template <std::size_t T1, typename T2, std::size_t T3>
using t14 = typename std::tuple_element<T1, std::array<T2, T3>>::type;

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
