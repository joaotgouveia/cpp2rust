#include <cstddef>
#include <sstream>
#include <utility>

template <typename T1, typename T2> long f11(const std::num_put<T1, T2> &a0) {
  return a0.use_count();
}
