#include <algorithm>
#include <tuple>
#include <utility>

template <typename... T1, typename T2 = typename std::enable_if<std::__and_<
                              std::__is_tuple_like<T1>...>::value>::type>
auto f71(T1 &&...a0)
    -> decltype(std::tuple_cat<T1...>(std::forward<T1>(a0)...)) {
  return std::tuple_cat<T1...>(std::forward<T1>(a0)...);
}
