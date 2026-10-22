#include "../common/init.h"
#include "../common/traits.h"
#include <functional>
#include <utility>

template <typename T1> auto f1(T1 &&a0) {
  return std::not_fn<T1>(std::forward<T1>(a0));
}
