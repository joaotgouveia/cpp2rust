#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
auto f1(const std::multiset<T1, T2, T3> &a0,
        const std::multiset<T1, T2, T3> &a1) {
  return std::operator<=> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f2(const std::multiset<T1, T2, T3> &a0,
        const std::multiset<T1, T2, T3> &a1) {
  return std::operator== <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
void f3(std::multiset<T1, T2, T3> &a0, std::multiset<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}
