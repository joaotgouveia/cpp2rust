#include <condition_variable>
#include <memory>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
std::weak_ptr<T1> f3(std::enable_shared_from_this<T1> &a0) {
  return a0.weak_from_this();
}

template <typename T1>
std::weak_ptr<const T1> f4(const std::enable_shared_from_this<T1> &a0) {
  return a0.weak_from_this();
}
