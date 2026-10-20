#include "../common/init.h"
#include "../common/traits.h"
#include <condition_variable>
#include <utility>

template <typename T1>
std::lock_guard<T1> f2(typename std::lock_guard<T1>::mutex_type &a0) {
  return std::lock_guard<T1>(a0);
}
