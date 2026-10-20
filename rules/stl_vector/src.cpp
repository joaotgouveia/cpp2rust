#include "../common/init.h"
#include "../common/traits.h"
#include <utility>
#include <vector>

template <typename T1, typename T2>
bool f1(const std::vector<T1, T2> &a0, const std::vector<T1, T2> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
void f2(std::vector<T1, T2> &a0, std::vector<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
