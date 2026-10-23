#include <chrono>
#include <type_traits>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::common_type<std::chrono::time_point<T1, T2>,
                            std::chrono::time_point<T1, T3>>;

template <typename T1, typename T2, typename T3, typename T4>
using t2 = std::common_type<std::chrono::duration<T1, T2>,
                            std::chrono::duration<T3, T4>>;
