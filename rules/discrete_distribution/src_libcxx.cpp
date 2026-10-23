#include <cstddef>
#include <initializer_list>
#include <istream>
#include <ostream>
#include <random>
#include <utility>
#include <vector>

template <typename T1>
bool f15(const std::discrete_distribution<T1> &a0,
         const std::discrete_distribution<T1> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f16(std::basic_ostream<T1, T2> &a0,
                                const std::discrete_distribution<T3> &a1) {
  return a0 << a1;
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f17(std::basic_istream<T1, T2> &a0,
                                std::discrete_distribution<T3> &a1) {
  return a0 >> a1;
}
