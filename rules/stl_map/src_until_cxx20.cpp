#include "../common/init.h"
#include "../common/traits.h"
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
bool f3(const std::map<T1, T2, T3, T4> &a0,
        const std::map<T1, T2, T3, T4> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f4(const std::map<T1, T2, T3, T4> &a0,
        const std::map<T1, T2, T3, T4> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f5(const std::map<T1, T2, T3, T4> &a0,
        const std::map<T1, T2, T3, T4> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f6(const std::map<T1, T2, T3, T4> &a0,
        const std::map<T1, T2, T3, T4> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f7(const std::map<T1, T2, T3, T4> &a0,
        const std::map<T1, T2, T3, T4> &a1) {
  return a0 >= a1;
}
