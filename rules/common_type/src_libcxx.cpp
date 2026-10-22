#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <type_traits>
#include <utility>

template <typename T1, typename T2, typename T3>
using t34 = typename std::common_type<std::chrono::time_point<T1, T2>,
                                      std::chrono::time_point<T1, T3>>::type;

template <typename T1, typename T2, typename T3>
using t35 =
    typename std::common_type<std::chrono::time_point<T1, T2>,
                              std::chrono::time_point<T1, T3>>::type::clock;

template <typename T1, typename T2, typename T3>
using t36 =
    typename std::common_type<std::chrono::time_point<T1, T2>,
                              std::chrono::time_point<T1, T3>>::type::duration;

template <typename T1, typename T2, typename T3>
using t37 =
    typename std::common_type<std::chrono::time_point<T1, T2>,
                              std::chrono::time_point<T1, T3>>::type::rep;

template <typename T1, typename T2, typename T3>
using t38 =
    typename std::common_type<std::chrono::time_point<T1, T2>,
                              std::chrono::time_point<T1, T3>>::type::period;

template <typename T1, typename T2, typename T3, typename T4>
using t39 = typename std::common_type<std::chrono::duration<T1, T2>,
                                      std::chrono::duration<T3, T4>>::type;

template <typename... T1> using t40 = std::common_type<T1...>;
