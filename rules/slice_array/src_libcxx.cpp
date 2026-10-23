#include <utility>
#include <valarray>

template <typename T1>
const std::slice_array<T1> &f25(const std::slice_array<T1> &a0,
                                const std::slice_array<T1> &a1) {
  return a0.operator=(a1);
}
