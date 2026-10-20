#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <barrier>
#include <syncstream>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &f67(std::basic_osyncstream<T1, T2, T3> &a0,
                                        const volatile void *a1) {
  return a0.operator<<(a1);
}
