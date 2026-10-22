#include "../common/init.h"
#include "../common/traits.h"
#include <istream>
#include <ostream>
#include <random>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f13(std::basic_ostream<T1, T2> &a0,
                                const std::student_t_distribution<T3> &a1) {
  return a0 << a1;
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f14(std::basic_istream<T1, T2> &a0,
                                std::student_t_distribution<T3> &a1) {
  return a0 >> a1;
}

template <typename T1>
std::student_t_distribution<T1>
f15(typename std::student_t_distribution<T1>::result_type a0) {
  return std::student_t_distribution<T1>(a0);
}

template <typename T1>
typename std::student_t_distribution<T1>::result_type
f16(const std::student_t_distribution<T1> &a0) {
  return a0.n();
}

template <typename T1>
bool f17(const std::student_t_distribution<T1> &a0,
         const std::student_t_distribution<T1> &a1) {
  return a0 != a1;
}
