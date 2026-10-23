#include <algorithm>
#include <utility>

template <typename T1> void f1(T1 a0, T1 a1) {
  return std::reverse<T1>(a0, a1);
}
