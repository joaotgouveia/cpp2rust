#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
auto f8(const std::multiset<T1, T2, T3> &a0,
        const std::multiset<T1, T2, T3> &a1) {
  return std::operator<=> <T1, T2, T3>(a0, a1);
}
