#include <algorithm>
#include <utility>

template <typename T1> std::pair<T1, T1> f1(T1 a0, T1 a1) {
  return std::minmax_element<T1>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> std::pair<T1, T1> f2(T1 a0, T1 a1, T2 a2) {
  return std::minmax_element<T1, T2>(std::move(a0), std::move(a1),
                                     std::move(a2));
}
