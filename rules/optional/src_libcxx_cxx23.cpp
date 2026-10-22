#include "../common/init.h"
#include "../common/traits.h"
#include <compare>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1, typename T2, typename T3, typename... T4>
std::optional<T1> f94(T2 a0, T3 &&a1, T4 &&...a2) {
  return std::optional<T1>(a0, std::forward<T3>(a1), std::forward<T4>(a2)...);
}
