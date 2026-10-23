#include <condition_variable>
#include <memory>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t1 = std::enable_shared_from_this<T1>;

template <typename T1>
std::shared_ptr<T1> f1(std::enable_shared_from_this<T1> &a0) {
  return a0.shared_from_this();
}

template <typename T1>
std::shared_ptr<const T1> f2(const std::enable_shared_from_this<T1> &a0) {
  return a0.shared_from_this();
}
