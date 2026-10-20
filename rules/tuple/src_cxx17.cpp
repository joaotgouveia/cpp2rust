#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <string>
#include <tuple>
#include <utility>

template <typename T1, typename T2> decltype(auto) f82(T1 &&a0, T2 &&a1) {
  return std::apply<T1, T2>(std::forward<T1>(a0), std::forward<T2>(a1));
}

template <typename T1, typename T2> T1 f83(T2 &&a0) {
  return std::make_from_tuple<T1, T2>(std::forward<T2>(a0));
}
