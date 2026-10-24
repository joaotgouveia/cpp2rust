#include <algorithm>
#include <utility>

template <typename T1, typename T2>
typename std::iterator_traits<T1>::difference_type f1(T1 a0, T1 a1,
                                                      const T2 &a2) {
  return std::count<T1, T2>(std::move(a0), std::move(a1), a2);
}
