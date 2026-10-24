#include <algorithm>
#include <utility>

template <typename T1, typename T2> T2 f1(T1 a0, T1 a1, T2 a2) {
  return std::copy_backward<T1, T2>(std::move(a0), std::move(a1),
                                    std::move(a2));
}
