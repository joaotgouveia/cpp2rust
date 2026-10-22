#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <utility>

template <typename T1, typename T2>
std::chrono::duration<T1, T2> f24(std::chrono::duration<T1, T2> a0) {
  return std::chrono::abs<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3>
std::chrono::time_point<T2, T1> f25(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::ceil<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::chrono::time_point<T2, T1> f26(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::floor<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::chrono::time_point<T2, T1> f27(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::round<T1, T2, T3>(a0);
}
