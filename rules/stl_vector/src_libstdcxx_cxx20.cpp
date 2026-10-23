#include <utility>
#include <vector>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
auto f8(const std::vector<T1, T2> &a0, const std::vector<T1, T2> &a1) {
  return a0 <=> a1;
}
