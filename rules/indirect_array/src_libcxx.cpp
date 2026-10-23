#include <utility>
#include <valarray>

template <typename T1>
const std::indirect_array<T1> &f15(const std::indirect_array<T1> &a0,
                                   const std::indirect_array<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
void f16(const std::indirect_array<T1> &a0,
         const typename std::indirect_array<T1>::value_type &a1) {
  return a0.operator=(a1);
}
