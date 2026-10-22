#include "../common/init.h"
#include "../common/traits.h"
#include <queue>
#include <utility>

template <typename T1, typename T2, typename T3>
std::priority_queue<T1, T2, T3>
f33(const typename std::priority_queue<T1, T2, T3>::value_compare &a0) {
  return std::priority_queue<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::priority_queue<T1, T2, T3>
f34(const typename std::priority_queue<T1, T2, T3>::value_compare &a0,
    const typename std::priority_queue<T1, T2, T3>::container_type &a1) {
  return std::priority_queue<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::priority_queue<T1, T2, T3>
f35(const typename std::priority_queue<T1, T2, T3>::value_compare &a0,
    typename std::priority_queue<T1, T2, T3>::container_type &&a1) {
  return std::priority_queue<T1, T2, T3>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::priority_queue<T1, T2, T3>
f36(T4 a0, T4 a1,
    const typename std::priority_queue<T1, T2, T3>::value_compare &a2) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::priority_queue<T1, T2, T3>
f37(T4 a0, T4 a1,
    const typename std::priority_queue<T1, T2, T3>::value_compare &a2,
    const typename std::priority_queue<T1, T2, T3>::container_type &a3) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::priority_queue<T1, T2, T3>
f38(T4 a0, T4 a1,
    const typename std::priority_queue<T1, T2, T3>::value_compare &a2,
    typename std::priority_queue<T1, T2, T3>::container_type &&a3) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2, std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
std::priority_queue<T1, T2, T3>
f39(const typename std::priority_queue<T1, T2, T3>::value_compare &a0,
    const T4 &a1) {
  return std::priority_queue<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::priority_queue<T1, T2, T3>
f40(const typename std::priority_queue<T1, T2, T3>::value_compare &a0,
    const typename std::priority_queue<T1, T2, T3>::container_type &a1,
    const T4 &a2) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::priority_queue<T1, T2, T3>
f41(const typename std::priority_queue<T1, T2, T3>::value_compare &a0,
    typename std::priority_queue<T1, T2, T3>::container_type &&a1,
    const T4 &a2) {
  return std::priority_queue<T1, T2, T3>(a0, std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::priority_queue<T1, T2, T3>
f42(T4 a0, T4 a1,
    const typename std::priority_queue<T1, T2, T3>::value_compare &a2,
    const T5 &a3) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::priority_queue<T1, T2, T3>
f43(T4 a0, T4 a1,
    const typename std::priority_queue<T1, T2, T3>::value_compare &a2,
    const typename std::priority_queue<T1, T2, T3>::container_type &a3,
    const T5 &a4) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::priority_queue<T1, T2, T3>
f44(T4 a0, T4 a1,
    const typename std::priority_queue<T1, T2, T3>::value_compare &a2,
    typename std::priority_queue<T1, T2, T3>::container_type &&a3,
    const T5 &a4) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2, std::move(a3), a4);
}
