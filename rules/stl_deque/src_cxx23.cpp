#include <deque>
#include <utility>

template <typename T1, typename T2>
auto f1(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return std::operator<=>(a0, a1);
}

template <typename T1, typename T2>
bool f2(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return std::operator==(a0, a1);
}

template <typename T1, typename T2>
void f3(std::deque<T1, T2> &a0, std::deque<T1, T2> &a1) {
  return std::swap(a0, a1);
}
