#include <iterator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3, typename T4>
std::istream_iterator<T1, T2, T3, T4>
f13(const std::istream_iterator<T1, T2, T3, T4> &a0) {
  return std::istream_iterator<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::istream_iterator<T1, T2, T3, T4> &
f14(std::istream_iterator<T1, T2, T3, T4> &a0,
    const std::istream_iterator<T1, T2, T3, T4> &a1) {
  return a0.operator=(a1);
}
