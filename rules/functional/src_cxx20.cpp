#include "../common/init.h"
#include "../common/traits.h"
#include <functional>
#include <type_traits>
#include <utility>

template <typename T1, typename... T2> auto f6(T1 &&a0, T2 &&...a1) {
  return std::bind_front<T1, T2...>(std::forward<T1>(a0),
                                    std::forward<T2>(a1)...);
}
