#include <algorithm>
#include <utility>

template <typename T1> std::reverse_iterator<T1> f62(T1 a0) {
  return std::make_reverse_iterator<T1>(std::move(a0));
}
