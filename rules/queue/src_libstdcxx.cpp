#include <queue>
#include <utility>

template <typename T1, typename T2, typename T3 = T2,
          typename T4 = typename std::enable_if<
              std::is_default_constructible<T2>::value>::type>
std::queue<T1, T2> f17() {
  return std::queue<T1, T2>();
}

template <typename T1, typename T2, typename T3,
          typename T4 =
              typename std::enable_if<std::uses_allocator<T2, T3>::value>::type>
std::queue<T1, T2> f18(const T2 &a0, const T3 &a1) {
  return std::queue<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          typename T4 =
              typename std::enable_if<std::uses_allocator<T2, T3>::value>::type>
std::queue<T1, T2> f19(T2 &&a0, const T3 &a1) {
  return std::queue<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3,
          typename T4 =
              typename std::enable_if<std::uses_allocator<T2, T3>::value>::type>
std::queue<T1, T2> f20(const std::queue<T1, T2> &a0, const T3 &a1) {
  return std::queue<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          typename T4 =
              typename std::enable_if<std::uses_allocator<T2, T3>::value>::type>
std::queue<T1, T2> f21(std::queue<T1, T2> &&a0, const T3 &a1) {
  return std::queue<T1, T2>(std::move(a0), a1);
}
