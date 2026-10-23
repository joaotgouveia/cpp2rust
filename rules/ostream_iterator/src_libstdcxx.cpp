#include <iterator>
#include <ostream>
#include <utility>

template <typename T1, typename T2, typename T3>
std::ostream_iterator<T1, T2, T3>
f7(const std::ostream_iterator<T1, T2, T3> &a0) {
  return std::ostream_iterator<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::ostream_iterator<T1, T2, T3> &
f8(std::ostream_iterator<T1, T2, T3> &a0,
   const std::ostream_iterator<T1, T2, T3> &a1) {
  return a0.operator=(a1);
}
