#include "../common/init.h"
#include "../common/traits.h"
#include <istream>
#include <ostream>
#include <random>
#include <utility>

template <typename T1>
typename std::normal_distribution<T1>::result_type
f13(const std::normal_distribution<T1> &a0) {
  return a0.mean();
}

template <typename T1>
typename std::normal_distribution<T1>::result_type
f14(const std::normal_distribution<T1> &a0) {
  return a0.stddev();
}

template <typename T1>
bool f15(const std::normal_distribution<T1> &a0,
         const std::normal_distribution<T1> &a1) {
  return a0 != a1;
}

template <typename T1>
bool f16(const std::normal_distribution<T1> &a0,
         const std::normal_distribution<T1> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f17(std::basic_ostream<T1, T2> &a0,
                                const std::normal_distribution<T3> &a1) {
  return a0 << a1;
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f18(std::basic_istream<T1, T2> &a0,
                                std::normal_distribution<T3> &a1) {
  return a0 >> a1;
}
