#include <utility>
#include <valarray>

template <typename T1> using t1 = std::mask_array<T1>;

template <typename T1> using t2 = typename std::mask_array<T1>::value_type;

template <typename T1> std::mask_array<T1> f1(const std::mask_array<T1> &a0) {
  return std::mask_array<T1>(a0);
}
