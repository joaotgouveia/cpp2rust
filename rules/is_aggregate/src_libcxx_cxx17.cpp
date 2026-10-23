#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
typename std::is_aggregate<T1>::value_type f1(const std::is_aggregate<T1> &a0) {
  return static_cast<typename std::is_aggregate<T1>::value_type>(a0);
}

template <typename T1>
typename std::is_aggregate<T1>::value_type f2(const std::is_aggregate<T1> &a0) {
  return a0.operator()();
}
