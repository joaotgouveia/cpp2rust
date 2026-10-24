#include <compare>
#include <iterator>
#include <utility>

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f40(const std::reverse_iterator<T1> &a0,
                                            const std::reverse_iterator<T2> &a1)
  requires(std::three_way_comparable_with<T2, T1>)
{
  return std::operator<=> <T1, T2>(a0, a1);
}
