#include "../common/init.h"
#include "../common/traits.h"
#include <queue>
#include <utility>

template <typename T1, typename T2>
bool f1(const std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f2(const std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f3(const std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f4(const std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
bool f5(const std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f6(const std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2>
void f7(std::queue<T1, T2> &a0, std::queue<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
void f8(std::priority_queue<T1, T2, T3> &a0,
        std::priority_queue<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}
