#include <memory>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
std::raw_storage_iterator<T1, T2> &f8(std::raw_storage_iterator<T1, T2> &a0,
                                      T2 &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
T1 f9(const std::raw_storage_iterator<T1, T2> &a0) {
  return a0.base();
}
