#include <numeric>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2> void f1(T1 a0, T1 a1, T2 a2) {
  return std::iota<T1, T2>(a0, a1, a2);
}
