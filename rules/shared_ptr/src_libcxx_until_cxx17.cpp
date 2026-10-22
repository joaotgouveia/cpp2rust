#include "../common/init.h"
#include "../common/traits.h"
#include <atomic>
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1> bool f100(const std::shared_ptr<T1> &a0) {
  return a0.unique();
}

template <typename T1, typename T2>
std::shared_ptr<T1> f101(std::auto_ptr<T2> &&a0) {
  return std::shared_ptr<T1>(std::move(a0));
}

template <typename T1, typename T2>
std::shared_ptr<T1> &f102(std::shared_ptr<T1> &a0, std::auto_ptr<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}
