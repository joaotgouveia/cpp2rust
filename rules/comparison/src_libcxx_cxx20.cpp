#include "../common/init.h"
#include "../common/traits.h"
#include <format>
#include <utility>
#include <vector>

template <typename T1, typename T2>
auto f7(const std::vector<T1, T2> &a0, const std::vector<T1, T2> &a1) {
  return a0 <=> a1;
}
