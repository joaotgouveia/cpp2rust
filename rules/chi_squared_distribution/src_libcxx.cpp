#include <istream>
#include <ostream>
#include <random>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
std::chi_squared_distribution<T1>
f13(typename std::chi_squared_distribution<T1>::result_type a0) {
  return std::chi_squared_distribution<T1>(a0);
}

template <typename T1>
typename std::chi_squared_distribution<T1>::result_type
f14(const std::chi_squared_distribution<T1> &a0) {
  return a0.n();
}

template <typename T1>
bool f15(const std::chi_squared_distribution<T1> &a0,
         const std::chi_squared_distribution<T1> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f16(std::basic_ostream<T1, T2> &a0,
                                const std::chi_squared_distribution<T3> &a1) {
  return a0 << a1;
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f17(std::basic_istream<T1, T2> &a0,
                                std::chi_squared_distribution<T3> &a1) {
  return a0 >> a1;
}
