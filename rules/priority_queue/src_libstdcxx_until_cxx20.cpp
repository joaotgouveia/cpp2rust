#include <queue>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 =
              typename std::enable_if<std::uses_allocator<T2, T4>::value>::type,
          typename std::enable_if<
              (!(std::is_same<T4, std::priority_queue<T1, T2, T3>>::value)),
              int>::type T6 = 0>
std::priority_queue<T1, T2, T3> f27(const T4 &a0) {
  return std::priority_queue<T1, T2, T3>(a0);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename T5 =
        typename std::enable_if<std::uses_allocator<T2, T4>::value>::type,
    typename std::enable_if<(!(std::is_same<T4, T2>::value)), int>::type T6 = 0>
std::priority_queue<T1, T2, T3> f28(const T3 &a0, const T4 &a1) {
  return std::priority_queue<T1, T2, T3>(a0, a1);
}
