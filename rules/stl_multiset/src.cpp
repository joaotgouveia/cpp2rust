#include "../common/init.h"
#include "../common/traits.h"
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f1(const std::multiset<T1, T2, T3> &a0,
        const std::multiset<T1, T2, T3> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3>
void f2(std::multiset<T1, T2, T3> &a0, std::multiset<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}
