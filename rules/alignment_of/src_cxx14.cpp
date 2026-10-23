#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
typename std::alignment_of<T1>::value_type f2(const std::alignment_of<T1> &a0) {
  return a0.operator()();
}
