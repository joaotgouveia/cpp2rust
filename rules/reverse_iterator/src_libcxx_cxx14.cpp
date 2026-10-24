#include <iterator>
#include <utility>

template <typename T1> std::reverse_iterator<T1> f39(T1 a0) {
  return std::make_reverse_iterator<T1>(std::move(a0));
}
