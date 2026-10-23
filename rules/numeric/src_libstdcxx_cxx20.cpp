#include <numeric>
#include <type_traits>
#include <utility>

template <typename T1>
std::enable_if_t<std::is_object_v<T1>, T1 *> f17(T1 *a0, T1 *a1) {
  return std::midpoint<T1>(a0, a1);
}

template <typename T1>
std::enable_if_t<
    std::conjunction_v<
        std::is_arithmetic<T1>, std::is_same<std::remove_cv_t<T1>, T1>,
        std::integral_constant<bool, !std::is_same<T1, bool>::value>>,
    T1>
f18(T1 a0, T1 a1) {
  return std::midpoint<T1>(a0, a1);
}
