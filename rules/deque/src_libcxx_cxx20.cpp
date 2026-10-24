#include <compare>
#include <deque>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
std::strong_ordering
f127(const std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a0,
     const std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, typename T2>
auto f128(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return std::operator<=> <T1, T2>(a0, a1);
}
