#include <chrono>
#include <limits>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
std::enable_if_t<std::numeric_limits<T1>::is_signed,
                 std::chrono::duration<T1, T2>>
f43(std::chrono::duration<T1, T2> a0) {
  return std::chrono::abs<T1, T2>(
      std::forward<std::chrono::duration<T1, T2>>(a0));
}

template <typename T1, typename T2, typename T3>
std::chrono::__enable_if_is_duration<T1>
f44(const std::chrono::duration<T2, T3> &a0) {
  return std::chrono::ceil<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::enable_if_t<std::chrono::__is_duration_v<T1>,
                 std::chrono::time_point<T2, T1>>
f45(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::ceil<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::chrono::__enable_if_is_duration<T1>
f46(const std::chrono::duration<T2, T3> &a0) {
  return std::chrono::floor<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::enable_if_t<std::chrono::__is_duration_v<T1>,
                 std::chrono::time_point<T2, T1>>
f47(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::floor<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::enable_if_t<std::__and_<std::chrono::__is_duration<T1>,
                             std::__not_<std::chrono::treat_as_floating_point<
                                 typename T1::rep>>>::value,
                 T1>
f48(const std::chrono::duration<T2, T3> &a0) {
  return std::chrono::round<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::enable_if_t<std::chrono::__is_duration_v<T1> &&
                     !std::chrono::treat_as_floating_point_v<typename T1::rep>,
                 std::chrono::time_point<T2, T1>>
f49(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::round<T1, T2, T3>(a0);
}
