#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f6(const std::set<T1, T2, T3> &a0, const std::set<T1, T2, T3> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3>
void f7(std::set<T1, T2, T3> &a0, std::set<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}
