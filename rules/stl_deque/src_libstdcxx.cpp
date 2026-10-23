#include <deque>
#include <utility>

template <typename T1, typename T2>
bool f6(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
void f7(std::deque<T1, T2> &a0, std::deque<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
