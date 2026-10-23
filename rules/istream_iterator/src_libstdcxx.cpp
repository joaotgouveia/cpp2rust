#include <istream>
#include <iterator>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
std::istream_iterator<T1, T2, T3, T4>
f9(const std::istream_iterator<T1, T2, T3, T4> &a0) {
  return std::istream_iterator<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::istream_iterator<T1, T2, T3, T4> &
f10(std::istream_iterator<T1, T2, T3, T4> &a0,
    const std::istream_iterator<T1, T2, T3, T4> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f11(const std::istream_iterator<T1, T2, T3, T4> &a0,
         const std::istream_iterator<T1, T2, T3, T4> &a1) {
  return operator==(a0, a1);
}
