#include <chrono>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
bool f42(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator!= <T1, T2, T3, T4>(a0, a1);
}
