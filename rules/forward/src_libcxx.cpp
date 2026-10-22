#include "../common/init.h"
#include "../common/traits.h"
#include <utility>

template <typename T1> T1 &&f1(__remove_reference_t(T1) &a0) {
  return std::forward<T1>(a0);
}

template <typename T1> T1 &&f2(__remove_reference_t(T1) &&a0) {
  return std::forward<T1>(std::move(a0));
}
