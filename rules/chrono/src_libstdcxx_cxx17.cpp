#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <utility>

template <typename T1, typename T2>
std::chrono::duration<T1, T2> f101(std::chrono::duration<T1, T2> a0) {
  return std::chrono::abs<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3>
T1 f102(const std::chrono::duration<T2, T3> &a0) {
  return std::chrono::ceil<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::chrono::time_point<T2, T1>
f103(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::ceil<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
T1 f104(const std::chrono::duration<T2, T3> &a0) {
  return std::chrono::floor<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::chrono::time_point<T2, T1>
f105(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::floor<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
T1 f106(const std::chrono::duration<T2, T3> &a0) {
  return std::chrono::round<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::chrono::time_point<T2, T1>
f107(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::round<T1, T2, T3>(a0);
}
