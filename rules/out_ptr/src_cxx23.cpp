#include <memory>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename... T3>
auto f1(T2 &a0, T3 &&...a1) {
  return std::out_ptr<T1, T2, T3...>(a0, std::forward<T3>(a1)...);
}
