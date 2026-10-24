#include <queue>
#include <utility>

template <typename T1, typename T2>
typename std::enable_if<std::__is_swappable<T2>::value>::type
f9(std::queue<T1, T2> &a0, std::queue<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<
    std::__and_<std::__is_swappable<T2>, std::__is_swappable<T3>>::value>::type
f10(std::priority_queue<T1, T2, T3> &a0, std::priority_queue<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}
