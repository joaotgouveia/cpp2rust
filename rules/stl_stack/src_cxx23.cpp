#include <compare>
#include <stack>
#include <utility>

template <typename T1, typename T2>
bool f1(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f2(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f3(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::compare_three_way_result_t<T2> f4(const std::stack<T1, T2> &a0,
                                       const std::stack<T1, T2> &a1)
  requires(std::three_way_comparable<T2>)
{
  return std::operator<=> <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f5(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f6(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f7(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
typename std::enable_if<std::is_swappable_v<T2>>::type
f8(std::stack<T1, T2> &a0, std::stack<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
