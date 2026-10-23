#include <chrono>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f35(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator!= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f36(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator!= <T1, T2, T3, T4>(a0, a1);
}
