#include <algorithm>
#include <array>
#include <tuple>
#include <utility>

template <typename T1, typename T2>
using t10 = typename std::tuple_element<0, std::pair<T1, T2>>::type;

template <typename T1, typename T2>
using t12 = typename std::tuple_element<1, std::pair<T1, T2>>::type;

template <std::size_t T1, typename T2, std::size_t T3>
using t14 = typename std::tuple_element<T1, std::array<T2, T3>>::type;
