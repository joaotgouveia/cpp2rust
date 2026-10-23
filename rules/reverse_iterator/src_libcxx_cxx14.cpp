#include <iterator>
#include <utility>

template <typename T1> std::reverse_iterator<T1> f32(T1 a0) {
  return std::make_reverse_iterator<T1>(a0);
}
