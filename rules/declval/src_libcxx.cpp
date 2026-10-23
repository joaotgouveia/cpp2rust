#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> auto f1() -> decltype(std::declval<T1>()) {
  return std::declval<T1>();
}
