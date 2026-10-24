#include <algorithm>
#include <any>
#include <initializer_list>
#include <type_traits>
#include <utility>

template <typename T1>
std::enable_if_t<
    std::is_copy_constructible<std::enable_if_t<
        !std::is_same_v<std::decay_t<T1>, std::any>, std::decay_t<T1>>>::value,
    std::any &>
f27(std::any &a0, T1 &&a1)
  requires(!(std::is_same<T1, const std::any &>::value) &&
           !(std::is_same<T1, std::any>::value))
{
  return a0.template operator= <T1>(std::forward<T1>(a1));
}
