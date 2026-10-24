#include <stack>
#include <utility>

template <typename T1, typename T2, typename T3 = T2,
          typename T4 = typename std::enable_if<
              std::is_default_constructible<T2>::value>::type>
std::stack<T1, T2> f13() {
  return std::stack<T1, T2>();
}

template <typename T1, typename T2, typename T3,
          typename T4 =
              typename std::enable_if<std::uses_allocator<T2, T3>::value>::type>
std::stack<T1, T2> f14(const T2 &a0, const T3 &a1) {
  return std::stack<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          typename T4 =
              typename std::enable_if<std::uses_allocator<T2, T3>::value>::type>
std::stack<T1, T2> f15(T2 &&a0, const T3 &a1) {
  return std::stack<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3,
          typename T4 =
              typename std::enable_if<std::uses_allocator<T2, T3>::value>::type>
std::stack<T1, T2> f16(const std::stack<T1, T2> &a0, const T3 &a1) {
  return std::stack<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          typename T4 =
              typename std::enable_if<std::uses_allocator<T2, T3>::value>::type>
std::stack<T1, T2> f17(std::stack<T1, T2> &&a0, const T3 &a1) {
  return std::stack<T1, T2>(std::move(a0), a1);
}
