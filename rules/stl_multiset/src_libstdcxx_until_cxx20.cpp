#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f1(const std::multiset<T1, T2, T3> &a0,
        const std::multiset<T1, T2, T3> &a1) {
  return std::operator!= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f2(const std::multiset<T1, T2, T3> &a0,
        const std::multiset<T1, T2, T3> &a1) {
  return std::operator< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f3(const std::multiset<T1, T2, T3> &a0,
        const std::multiset<T1, T2, T3> &a1) {
  return std::operator<= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f4(const std::multiset<T1, T2, T3> &a0,
        const std::multiset<T1, T2, T3> &a1) {
  return std::operator><T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f5(const std::multiset<T1, T2, T3> &a0,
        const std::multiset<T1, T2, T3> &a1) {
  return std::operator>= <T1, T2, T3>(a0, a1);
}
