#include <chrono>
#include <utility>

template <typename T1, typename T2,
          std::enable_if_t<std::numeric_limits<T1>::is_signed, int> T3 = 0>
std::chrono::duration<T1, T2> f25(std::chrono::duration<T1, T2> a0) {
  return std::chrono::abs<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3,
          std::enable_if_t<std::chrono::__is_duration_v<T1>, int> T4 = 0>
std::chrono::time_point<T2, T1> f26(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::ceil<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3,
          std::enable_if_t<std::chrono::__is_duration_v<T1>, int> T4 = 0>
std::chrono::time_point<T2, T1> f27(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::floor<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3,
          std::enable_if_t<std::chrono::__is_duration_v<T1>, int> T4 = 0>
std::chrono::time_point<T2, T1> f28(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::round<T1, T2, T3>(a0);
}
