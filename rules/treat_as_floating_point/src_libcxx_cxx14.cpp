#include <chrono>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
typename std::chrono::treat_as_floating_point<T1>::value_type
f2(const std::chrono::treat_as_floating_point<T1> &a0) {
  return a0.operator()();
}
