#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <utility>

template <typename T1>
typename std::chrono::treat_as_floating_point<T1>::value_type
f2(const std::chrono::treat_as_floating_point<T1> &a0) {
  return a0.operator()();
}
