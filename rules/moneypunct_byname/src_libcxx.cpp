#include <cstddef>
#include <iomanip>
#include <string.h>
#include <string>
#include <utility>

template <typename T1, bool T2>
using t4 = typename std::moneypunct_byname<T1, T2>::pattern;

template <typename T1, bool T2>
long f13(const std::moneypunct_byname<T1, T2> &a0) {
  return a0.use_count();
}
