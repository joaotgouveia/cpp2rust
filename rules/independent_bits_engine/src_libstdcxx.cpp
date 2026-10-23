#include <random>
#include <utility>

template <typename T1, std::size_t T2, typename T3>
bool f13(const std::independent_bits_engine<T1, T2, T3> &a0,
         const std::independent_bits_engine<T1, T2, T3> &a1) {
  return operator==(a0, a1);
}
