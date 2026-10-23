#include <chrono>
#include <type_traits>
#include <utility>

template <typename T1, typename T2, typename T3>
using t24 = typename std::common_type<std::chrono::time_point<T1, T2>,
                                      std::chrono::time_point<T1, T3>>::type;

template <typename T1, typename T2, typename T3, typename T4>
using t25 = typename std::common_type<std::chrono::duration<T1, T2>,
                                      std::chrono::duration<T3, T4>>::type;

template <typename... T1> using t26 = std::common_type<T1...>;
