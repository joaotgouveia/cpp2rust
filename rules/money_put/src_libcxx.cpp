#include "../common/init.h"
#include "../common/traits.h"
#include <cstddef>
#include <iomanip>
#include <ios>
#include <utility>

template <typename T1, typename T2> long f5(const std::money_put<T1, T2> &a0) {
  return a0.use_count();
}
