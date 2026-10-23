#include <chrono>
#include <type_traits>
#include <utility>

using t5 = std::common_type<>;

std::common_type<> f1() { return std::common_type<>(); }

std::common_type<> f2(const std::common_type<> &a0) {
  return std::common_type<>(a0);
}

std::common_type<> f3(std::common_type<> &&a0) {
  return std::common_type<>(std::move(a0));
}

std::common_type<> &f4(std::common_type<> &a0, const std::common_type<> &a1) {
  return a0.operator=(a1);
}

std::common_type<> &f5(std::common_type<> &a0, std::common_type<> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> using t6 = std::common_type<T1>;

template <typename T1, typename T2>
using t7 = std::common_type<std::chrono::duration<T1, T2>>;

template <typename T1, typename T2>
using t8 = typename std::common_type<std::chrono::duration<T1, T2>>::type;

template <typename T1, typename T2>
using t9 = std::common_type<std::chrono::time_point<T1, T2>>;

template <typename T1, typename T2>
using t10 = typename std::common_type<std::chrono::time_point<T1, T2>>::type;

template <typename T1, typename T2>
using t11 = std::common_type<std::chrono::duration<T1, T2>,
                             std::chrono::duration<T1, T2>>;

template <typename T1, typename T2>
using t12 = typename std::common_type<std::chrono::duration<T1, T2>,
                                      std::chrono::duration<T1, T2>>::type;

template <typename T1, typename T2>
using t13 = std::common_type<std::chrono::time_point<T1, T2>,
                             std::chrono::time_point<T1, T2>>;

template <typename T1, typename T2>
using t14 = typename std::common_type<std::chrono::time_point<T1, T2>,
                                      std::chrono::time_point<T1, T2>>::type;

template <typename T1, typename T2> using t15 = std::common_type<T1, T2>;

template <typename T1, typename T2, typename... T3>
using t16 = std::common_type<T1, T2, T3...>;
