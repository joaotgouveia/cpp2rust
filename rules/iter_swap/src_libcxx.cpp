#include <algorithm>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2> void f1(T1 a0, T2 a1) {
  return std::iter_swap<T1, T2>(a0, a1);
}
