#include <utility>

template <typename T1, typename T2> T1 f1(T1 &a0, T2 &&a1) {
  return std::exchange<T1, T2>(a0, std::forward<T2>(a1));
}
