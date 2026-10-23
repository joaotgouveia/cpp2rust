#include <algorithm>
#include <utility>

template <typename T1> bool f1(T1 a0, T1 a1) {
  return std::next_permutation<T1>(a0, a1);
}

template <typename T1, typename T2> bool f2(T1 a0, T1 a1, T2 a2) {
  return std::next_permutation<T1, T2>(a0, a1, a2);
}
