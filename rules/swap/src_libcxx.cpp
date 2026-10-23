#include <utility>
#include <vector>

template <typename T1, typename T2>
void f1(std::vector<T1, T2> &a0, std::vector<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, std::size_t T2,
          std::__enable_if_t<std::__is_swappable_v<T1>, int> T3 = 0>
void f2(T1 (&a0)[T2], T1 (&a1)[T2]) {
  return std::swap<T1, T2>(a0, a1);
}
