#include <cstddef>
#include <iomanip>
#include <ios>
#include <string.h>
#include <utility>

template <typename T1, typename T2>
long f5(const std::time_put_byname<T1, T2> &a0) {
  return a0.use_count();
}
