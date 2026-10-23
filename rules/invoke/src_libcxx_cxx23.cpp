#include <functional>
#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename... T3> T1 f2(T2 &&a0, T3 &&...a1) {
  return std::invoke_r<T1, T2, T3...>(std::forward<T2>(a0),
                                      std::forward<T3>(a1)...);
}
