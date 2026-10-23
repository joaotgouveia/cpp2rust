#include <algorithm>
#include <string>
#include <tuple>
#include <utility>

template <typename T1, typename T2> T1 f70(T2 &&a0) {
  return std::make_from_tuple<T1, T2>(std::forward<T2>(a0));
}
