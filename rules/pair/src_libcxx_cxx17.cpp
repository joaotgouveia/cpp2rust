#include <array>
#include <tuple>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <
    typename T1, typename T2,
    typename std::enable_if<std::is_swappable_v<T1> && std::is_swappable_v<T2>,
                            int>::type T3 = 0>
void f59(std::pair<T1, T2> &a0, std::pair<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
