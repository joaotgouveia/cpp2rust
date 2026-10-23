#include <functional>
#include <utility>

template <typename T1> using t4 = std::bit_not<T1>;

template <typename T1> T1 f8(const std::bit_not<T1> &a0, const T1 &a1) {
  return a0.operator()(a1);
}
