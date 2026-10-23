#include <utility>
#include <vector>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
typename std::vector<T1, T2>::size_type f1(std::vector<T1, T2> &a0,
                                           const T3 &a1) {
  return std::erase<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::vector<T1, T2>::size_type f2(std::vector<T1, T2> &a0, T3 a1) {
  return std::erase_if<T1, T2, T3>(a0, a1);
}
