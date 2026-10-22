#include "../common/init.h"
#include "../common/traits.h"
#include <cstddef>
#include <sstream>
#include <utility>

template <typename T1, typename T2> long f14(const std::num_get<T1, T2> &a0) {
  return a0.use_count();
}
