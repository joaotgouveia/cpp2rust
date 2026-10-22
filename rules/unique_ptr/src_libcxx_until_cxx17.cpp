#include "../common/init.h"
#include "../common/traits.h"
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1, typename T2, typename T3>
std::unique_ptr<T1, T2> f73(std::auto_ptr<T3> &&a0) {
  return std::unique_ptr<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::unique_ptr<T1, T2> &f74(std::unique_ptr<T1, T2> &a0,
                             std::auto_ptr<T3> a1) {
  return a0.template operator= <T3>(a1);
}
