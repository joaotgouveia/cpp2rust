#include <algorithm>
#include <utility>

template <typename T1>
std::reverse_iterator<T1> f22(const std::reverse_iterator<T1> &a0) {
  return std::reverse_iterator<T1>(a0);
}

template <typename T1>
std::reverse_iterator<T1> &f23(std::reverse_iterator<T1> &a0,
                               const std::reverse_iterator<T1> &a1) {
  return a0.operator=(a1);
}
