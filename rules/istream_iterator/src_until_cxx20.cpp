#include <iterator>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
bool f10(const std::istream_iterator<T1, T2, T3, T4> &a0,
         const std::istream_iterator<T1, T2, T3, T4> &a1) {
  return a0 != a1;
}
