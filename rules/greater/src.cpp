#include "../common/init.h"
#include "../common/traits.h"
#include <functional>
#include <string>
#include <utility>

template <typename T1> using t1 = std::greater<T1>;

template <typename T1>
bool f1(const std::greater<T1> &a0, const T1 &a1, const T1 &a2) {
  return a0.operator()(a1, a2);
}
