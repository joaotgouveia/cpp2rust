#include <deque>
#include <utility>

template <typename T1, typename T2>
auto f8(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return a0 <=> a1;
}
