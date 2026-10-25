#include <array>

template <typename T1>
std::array<T1, 0> f18(const std::__empty (&a0)[sizeof(T1[1])]) {
  return {a0};
}
