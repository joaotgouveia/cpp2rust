#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <utility>

template <typename T1, typename T2>
std::chrono::duration<T1, T2> f18(const std::chrono::duration<T1, T2> &a0) {
  return std::chrono::duration<T1, T2>(a0);
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &f19(std::chrono::duration<T1, T2> &a0,
                                   const std::chrono::duration<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::chrono::duration<typename std::common_type<
                          typename std::chrono::duration<T1, T2>::rep>::type,
                      typename std::chrono::duration<T1, T2>::period>
f20(const std::chrono::duration<T1, T2> &a0) {
  return a0.operator+();
}

template <typename T1, typename T2>
std::chrono::duration<typename std::common_type<
                          typename std::chrono::duration<T1, T2>::rep>::type,
                      typename std::chrono::duration<T1, T2>::period>
f21(const std::chrono::duration<T1, T2> &a0) {
  return a0.operator-();
}
