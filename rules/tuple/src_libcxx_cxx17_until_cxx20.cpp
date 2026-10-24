#include <array>
#include <tuple>
#include <utility>

template <typename T1, typename T2,
          typename T3 = std::enable_if_t<std::__can_make_from_tuple<T1, T2>>>
T1 f169(T2 &&a0) {
  return std::make_from_tuple<T1, T2>(std::forward<T2>(a0));
}
