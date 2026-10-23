#include <cstddef>
#include <iomanip>
#include <ios>
#include <string.h>
#include <utility>

template <typename T1, typename T2>
using t4 = typename std::time_get_byname<T1, T2>::dateorder;

template <typename T1, typename T2>
using t5 = typename std::time_get_byname<T1, T2>::string_type;

template <typename T1, typename T2>
long f11(const std::time_get_byname<T1, T2> &a0) {
  return a0.use_count();
}
