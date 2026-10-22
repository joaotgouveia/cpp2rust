#include "../common/init.h"
#include "../common/traits.h"
#include <queue>
#include <utility>

template <typename T1, typename T2> std::queue<T1, T2> f22(const T2 &a0) {
  return std::queue<T1, T2>(a0);
}

template <typename T1, typename T2> std::queue<T1, T2> f23(T2 &&a0) {
  return std::queue<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::queue<T1, T2> f24(const T2 &a0, const T3 &a1) {
  return std::queue<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::queue<T1, T2> f25(T2 &&a0, const T3 &a1) {
  return std::queue<T1, T2>(std::move(a0), a1);
}
