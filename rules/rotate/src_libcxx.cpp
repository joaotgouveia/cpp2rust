#include <algorithm>
#include <utility>

template <typename T1> T1 f1(T1 a0, T1 a1, T1 a2) {
  return std::rotate<T1>(std::move(a0), std::move(a1), std::move(a2));
}
