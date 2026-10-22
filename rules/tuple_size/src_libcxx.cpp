#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <array>
#include <tuple>
#include <type_traits>
#include <utility>

template <typename T1>
using t8 = std::tuple_size<std::__enable_if_tuple_size_imp<
    const T1, std::__enable_if_t<!std::is_volatile<T1>::value>,
    decltype(std::tuple_size<T1>::value)>>;

template <typename T1>
using t9 = std::tuple_size<std::__enable_if_tuple_size_imp<
    volatile T1, std::__enable_if_t<!std::is_const<T1>::value>,
    decltype(std::tuple_size<T1>::value)>>;

template <typename T1>
using t10 = std::tuple_size<std::__enable_if_tuple_size_imp<
    const volatile T1, decltype(std::tuple_size<T1>::value)>>;
