#include "../common/init.h"
#include "../common/traits.h"
#include <queue>
#include <utility>
#include <vector>

template <typename T1> void f1(T1 &a0, T1 &a1) { return std::swap<T1>(a0, a1); }

template <typename T1, typename T2>
void f2(std::vector<T1, T2> &a0, std::vector<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, std::size_t T2> void f3(T1 (&a0)[T2], T1 (&a1)[T2]) {
  return std::swap<T1, T2>(a0, a1);
}
