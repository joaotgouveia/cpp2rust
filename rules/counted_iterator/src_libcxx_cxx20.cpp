#include <iterator>
#include <utility>

template <typename T1> decltype(auto) f26(const std::counted_iterator<T1> &a0) {
  return iter_move(a0);
}
