#include <chrono>
#include <utility>

template <typename T1, typename T2, typename T3,
          std::enable_if_t<std::chrono::__is_duration_v<T1>, int> T4 = 0>
T1 f45(const std::chrono::duration<T2, T3> &a0) {
  return std::chrono::ceil<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3,
          std::enable_if_t<std::chrono::__is_duration_v<T1>, int> T4 = 0>
T1 f46(const std::chrono::duration<T2, T3> &a0) {
  return std::chrono::floor<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3,
          std::enable_if_t<std::chrono::__is_duration_v<T1>, int> T4 = 0>
T1 f47(const std::chrono::duration<T2, T3> &a0) {
  return std::chrono::round<T1, T2, T3>(a0);
}
