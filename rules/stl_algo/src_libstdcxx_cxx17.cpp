#include <algorithm>
#include <utility>

template <typename T1>
const T1 &f105(const T1 &a0, const T1 &a1, const T1 &a2) {
  return std::clamp<T1>(a0, a1, a2);
}

template <typename T1, typename T2>
const T1 &f106(const T1 &a0, const T1 &a1, const T1 &a2, T2 a3) {
  return std::clamp<T1, T2>(a0, a1, a2, std::move(a3));
}

template <typename T1, typename T2, typename T3> T1 f107(T1 a0, T2 a1, T3 a2) {
  return std::for_each_n<T1, T2, T3>(std::move(a0), std::move(a1),
                                     std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
T2 f108(T1 a0, T1 a1, T2 a2, T3 a3, T4 &&a4) {
  return std::sample<T1, T2, T3, T4>(std::move(a0), std::move(a1),
                                     std::move(a2), std::move(a3),
                                     std::forward<T4>(a4));
}

template <typename T1, typename T2> T1 f109(T1 a0, T1 a1, const T2 &a2) {
  return std::search<T1, T2>(std::move(a0), std::move(a1), a2);
}
