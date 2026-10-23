#include <utility>
#include <valarray>

template <typename T1> using t1 = std::gslice_array<T1>;

template <typename T1> using t2 = typename std::gslice_array<T1>::value_type;

template <typename T1>
std::gslice_array<T1> f1(const std::gslice_array<T1> &a0) {
  return std::gslice_array<T1>(a0);
}

template <typename T1> void f2(const std::gslice_array<T1> &a0, const T1 &a1) {
  return a0.operator=(a1);
}
