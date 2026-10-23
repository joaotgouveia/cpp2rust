#include <utility>
#include <vector>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
bool f6(const std::vector<T1, T2> &a0, const std::vector<T1, T2> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
void f7(std::vector<T1, T2> &a0, std::vector<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
