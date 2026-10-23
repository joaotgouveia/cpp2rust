#include <chrono>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
bool f37(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return a0 != a1;
}
