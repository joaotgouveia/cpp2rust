#include <compare>
#include <stack>
#include <utility>

template <typename T1, typename T2>
std::compare_three_way_result_t<T2> f9(const std::stack<T1, T2> &a0,
                                       const std::stack<T1, T2> &a1) {
  return std::operator<=> <T1, T2>(a0, a1);
}
