#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1, typename T2>
std::pair<T1, T2> &f12(std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::pair<T1, T2> &f13(std::pair<T1, T2> &a0, std::pair<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}
