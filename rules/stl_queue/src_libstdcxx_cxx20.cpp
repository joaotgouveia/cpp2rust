#include <compare>
#include <queue>
#include <utility>

template <typename T1, typename T2>
std::compare_three_way_result_t<T2> f11(const std::queue<T1, T2> &a0,
                                        const std::queue<T1, T2> &a1) {
  return std::operator<=> <T1, T2>(a0, a1);
}
