#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
auto f1(const std::map<T1, T2, T3, T4> &a0,
        const std::map<T1, T2, T3, T4> &a1) {
  return std::operator<=> <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f2(const std::map<T1, T2, T3, T4> &a0,
        const std::map<T1, T2, T3, T4> &a1) {
  return std::operator== <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f3(std::map<T1, T2, T3, T4> &a0, std::map<T1, T2, T3, T4> &a1) {
  return std::swap<T1, T2, T3, T4>(a0, a1);
}
