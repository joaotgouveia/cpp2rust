#include <cstddef>
#include <iomanip>
#include <ios>
#include <string.h>
#include <string>
#include <utility>

template <typename T1, bool T2> long f13(const std::moneypunct<T1, T2> &a0) {
  return a0.use_count();
}
