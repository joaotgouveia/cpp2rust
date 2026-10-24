#include <algorithm>
#include <string>
#include <tuple>
#include <utility>

template <typename T1, typename T2> T1 f72(T2 &&a0) {
  return std::make_from_tuple<T1, T2>(std::forward<T2>(a0));
}

template <typename... T1>
typename std::enable_if<std::__and_<std::__is_swappable<T1>...>::value>::type
f73(std::tuple<T1...> &a0, std::tuple<T1...> &a1) {
  return std::swap<T1...>(a0, a1);
}
