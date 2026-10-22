#include "../common/init.h"
#include "../common/traits.h"
#include <functional>
#include <string>
#include <utility>

template <typename T1> using t1 = std::negate<T1>;

template <typename T1> T1 f1(const std::negate<T1> &a0, const T1 &a1) {
  return a0.operator()(a1);
}
