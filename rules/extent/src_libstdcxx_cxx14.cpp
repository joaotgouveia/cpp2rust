#include <type_traits>
#include <utility>

template <typename T1, unsigned int T2>
unsigned long f2(const std::extent<T1, T2> &a0) {
  return a0.operator()();
}
