#include <functional>
#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename... T2> auto f7(T1 &&a0, T2 &&...a1) {
  return std::bind_back<T1, T2...>(std::forward<T1>(a0),
                                   std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3> T1 f8(T2 &&a0, T3 &&...a1) {
  return std::invoke_r<T1, T2, T3...>(std::forward<T2>(a0),
                                      std::forward<T3>(a1)...);
}
