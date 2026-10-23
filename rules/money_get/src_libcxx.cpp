#include <cstddef>
#include <iomanip>
#include <ios>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2> long f5(const std::money_get<T1, T2> &a0) {
  return a0.use_count();
}
