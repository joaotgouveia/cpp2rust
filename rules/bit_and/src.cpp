#include <functional>
#include <string>
#include <utility>

template <typename T1> using t1 = std::bit_and<T1>;

template <typename T1>
T1 f1(const std::bit_and<T1> &a0, const T1 &a1, const T1 &a2) {
  return a0.operator()(a1, a2);
}
