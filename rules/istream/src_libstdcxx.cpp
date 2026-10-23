#include <istream>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2> T1 &&f11(T1 &&a0, T2 &&a1) {
  return std::forward<T1>(a0) >> std::forward<T2>(a1);
}
