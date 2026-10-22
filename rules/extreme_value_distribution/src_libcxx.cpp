#include "../common/init.h"
#include "../common/traits.h"
#include <istream>
#include <ostream>
#include <random>
#include <utility>

template <typename T1>
std::extreme_value_distribution<T1>
f14(typename std::extreme_value_distribution<T1>::result_type a0,
    typename std::extreme_value_distribution<T1>::result_type a1) {
  return std::extreme_value_distribution<T1>(a0, a1);
}

template <typename T1>
typename std::extreme_value_distribution<T1>::result_type
f15(const std::extreme_value_distribution<T1> &a0) {
  return a0.a();
}

template <typename T1>
typename std::extreme_value_distribution<T1>::result_type
f16(const std::extreme_value_distribution<T1> &a0) {
  return a0.b();
}

template <typename T1>
bool f17(const std::extreme_value_distribution<T1> &a0,
         const std::extreme_value_distribution<T1> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f18(std::basic_ostream<T1, T2> &a0,
                                const std::extreme_value_distribution<T3> &a1) {
  return a0 << a1;
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f19(std::basic_istream<T1, T2> &a0,
                                std::extreme_value_distribution<T3> &a1) {
  return a0 >> a1;
}
