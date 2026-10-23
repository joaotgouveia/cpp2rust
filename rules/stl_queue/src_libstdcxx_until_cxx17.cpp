#include <queue>
#include <utility>

template <typename T1, typename T2>
void f7(std::queue<T1, T2> &a0, std::queue<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
void f8(std::priority_queue<T1, T2, T3> &a0,
        std::priority_queue<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}
