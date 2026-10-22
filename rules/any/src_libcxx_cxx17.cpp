#include "../common/init.h"
#include "../common/traits.h"
#include <any>
#include <initializer_list>
#include <type_traits>
#include <utility>

template <typename T1, typename... T2> auto f34(std::any &a0, T2 &&...a1) {
  return a0.template emplace<T1, T2...>(std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3>
auto f35(std::any &a0, std::initializer_list<T2> a1, T3 &&...a2) {
  return a0.template emplace<T1, T2, T3...>(a1, std::forward<T3>(a2)...);
}

template <typename T1> std::add_pointer_t<T1> f36(std::any *a0) {
  return std::any_cast<T1>(a0);
}

template <typename T1>
std::add_pointer_t<std::add_const_t<T1>> f37(const std::any *a0) {
  return std::any_cast<T1>(a0);
}
