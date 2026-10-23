#include <cstddef>
#include <memory>
#include <utility>

template <typename T1, typename T2,
          typename std::enable_if<std::is_swappable_v<T2>, int>::type T3 = 0>
void f70(std::unique_ptr<T1, T2> &a0, std::unique_ptr<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
