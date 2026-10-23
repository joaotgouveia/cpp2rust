#include <utility>
#include <valarray>

template <typename T1>
const std::mask_array<T1> &f15(const std::mask_array<T1> &a0,
                               const std::mask_array<T1> &a1) {
  return a0.operator=(a1);
}
