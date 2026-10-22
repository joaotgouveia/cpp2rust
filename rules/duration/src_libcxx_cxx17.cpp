#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <utility>

template <typename T1, typename T2, typename T3>
T1 f39(const std::chrono::duration<T2, T3> &a0) {
  return std::chrono::ceil<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
T1 f40(const std::chrono::duration<T2, T3> &a0) {
  return std::chrono::floor<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
T1 f41(const std::chrono::duration<T2, T3> &a0) {
  return std::chrono::round<T1, T2, T3>(a0);
}
