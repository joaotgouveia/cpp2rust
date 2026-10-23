#include <algorithm>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, std::size_t T2>
typename std::enable_if<std::is_swappable<T1>::value>::type f6(T1 (&a0)[T2],
                                                               T1 (&a1)[T2]) {
  return std::swap<T1, T2>(a0, a1);
}
