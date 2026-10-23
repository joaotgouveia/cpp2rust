#include <algorithm>
#include <array>
#include <string>
#include <tuple>
#include <utility>

template <typename T1, typename T2> decltype(auto) f29(T1 &&a0, T2 &&a1) {
  return std::apply<T1, T2>(std::forward<T1>(a0), std::forward<T2>(a1));
}
