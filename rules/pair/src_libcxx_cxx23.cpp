#include "../common/init.h"
#include "../common/traits.h"
#include <tuple>
#include <utility>

template <typename T1, typename T2>
void f61(const std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3 = void>
const std::pair<T1, T2> &f62(const std::pair<T1, T2> &a0,
                             const std::pair<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3 = void>
const std::pair<T1, T2> &f63(const std::pair<T1, T2> &a0,
                             std::pair<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}
