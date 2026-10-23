#include <algorithm>
#include <tuple>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

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
