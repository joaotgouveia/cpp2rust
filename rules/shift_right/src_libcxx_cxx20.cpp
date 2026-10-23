#include <algorithm>
#include <utility>

template <typename T1>
T1 f1(T1 a0, T1 a1, typename std::iterator_traits<T1>::difference_type a2) {
  return std::shift_right<T1>(a0, a1, a2);
}
