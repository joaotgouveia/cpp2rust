#include <queue>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
std::priority_queue<T1, T2, T3> f33() {
  return std::priority_queue<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::priority_queue<T1, T2, T3>
f34(const typename std::priority_queue<T1, T2, T3>::value_compare &a0) {
  return std::priority_queue<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::priority_queue<T1, T2, T3>
f35(const typename std::priority_queue<T1, T2, T3>::value_compare &a0,
    const typename std::priority_queue<T1, T2, T3>::container_type &a1) {
  return std::priority_queue<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::priority_queue<T1, T2, T3>
f36(const typename std::priority_queue<T1, T2, T3>::value_compare &a0,
    typename std::priority_queue<T1, T2, T3>::container_type &&a1) {
  return std::priority_queue<T1, T2, T3>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<std::uses_allocator<T2, T4>::value, int>::type
              T5 = 0>
std::priority_queue<T1, T2, T3> f37(const T4 &a0) {
  return std::priority_queue<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<std::uses_allocator<T2, T4>::value, int>::type
              T5 = 0>
std::priority_queue<T1, T2, T3>
f38(const typename std::priority_queue<T1, T2, T3>::value_compare &a0,
    const T4 &a1) {
  return std::priority_queue<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<std::uses_allocator<T2, T4>::value, int>::type
              T5 = 0>
std::priority_queue<T1, T2, T3>
f39(const typename std::priority_queue<T1, T2, T3>::value_compare &a0,
    const typename std::priority_queue<T1, T2, T3>::container_type &a1,
    const T4 &a2) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<std::uses_allocator<T2, T4>::value, int>::type
              T5 = 0>
std::priority_queue<T1, T2, T3> f40(const std::priority_queue<T1, T2, T3> &a0,
                                    const T4 &a1) {
  return std::priority_queue<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<std::uses_allocator<T2, T4>::value, int>::type
              T5 = 0>
std::priority_queue<T1, T2, T3>
f41(const typename std::priority_queue<T1, T2, T3>::value_compare &a0,
    typename std::priority_queue<T1, T2, T3>::container_type &&a1,
    const T4 &a2) {
  return std::priority_queue<T1, T2, T3>(a0, std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<std::uses_allocator<T2, T4>::value, int>::type
              T5 = 0>
std::priority_queue<T1, T2, T3> f42(std::priority_queue<T1, T2, T3> &&a0,
                                    const T4 &a1) {
  return std::priority_queue<T1, T2, T3>(std::move(a0), a1);
}
