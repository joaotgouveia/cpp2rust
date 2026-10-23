#include <algorithm>
#include <utility>

template <typename T1, typename T2>
void f17(std::pair<T1, T2> &a0, std::pair<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
