#include <type_traits>
#include <utility>

template <typename T1, T1 T2> T1 f3(const std::integral_constant<T1, T2> &a0) {
  return a0.operator()();
}
