#include "../common/init.h"
#include "../common/traits.h"
#include <string>
#include <utility>

template <typename T1, typename T2>
bool f8(const std::greater_equal<void> &a0, T1 *a1, T2 *a2) {
  return a0.template operator()<T1, T2>(a1, a2);
}
