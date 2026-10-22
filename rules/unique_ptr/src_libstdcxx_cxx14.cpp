#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <condition_variable>
#include <utility>

template <typename T1> auto f51(std::size_t a0) {
  return std::make_unique<T1>(a0);
}

template <typename T1, typename... T2> std::unique_ptr<T1> f52(T2 &&...a0) {
  return std::make_unique<T1, T2...>(std::forward<T2>(a0)...);
}
