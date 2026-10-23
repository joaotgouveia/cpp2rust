#include <algorithm>
#include <utility>

template <typename T1, typename T2>
std::move_iterator<T1> f24(const std::move_iterator<T2> &a0) {
  return std::move_iterator<T1>(a0);
}

template <typename T1, typename T2>
std::move_iterator<T1> &f25(std::move_iterator<T1> &a0,
                            const std::move_iterator<T2> &a1) {
  return a0.template operator= <T2>(a1);
}
