#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <compare>
#include <type_traits>
#include <utility>
#include <variant>

template <typename... T1> std::variant<T1...> f36() {
  return std::variant<T1...>();
}

template <typename T1, typename T2, typename... T3>
T1 f37(T2 &&a0, T3 &&...a1) {
  return std::visit<T1, T2, T3...>(std::forward<T2>(a0),
                                   std::forward<T3>(a1)...);
}
