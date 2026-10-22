#include "../common/init.h"
#include "../common/traits.h"
#include <utility>
#include <vector>

template <typename T1, std::size_t T2,
          typename std::enable_if<std::is_swappable_v<T1>, int>::type T3 = 0>
void f3(T1 (&a0)[T2], T1 (&a1)[T2]) {
  return std::swap<T1, T2>(a0, a1);
}
