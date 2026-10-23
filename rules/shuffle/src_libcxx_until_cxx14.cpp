#include <algorithm>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> void f1(T1 a0, T1 a1) {
  return std::random_shuffle<T1>(a0, a1);
}

template <typename T1, typename T2> void f2(T1 a0, T1 a1, T2 &&a2) {
  return std::random_shuffle<T1, T2>(a0, a1, std::forward<T2>(a2));
}
