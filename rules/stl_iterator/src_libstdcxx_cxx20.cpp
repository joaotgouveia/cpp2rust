#include <algorithm>
#include <utility>

template <typename T1, typename T2>
using t8 = typename __gnu_cxx::__normal_iterator<T1, T2>::iterator_concept;

template <typename T1, typename T2, typename T3>
__gnu_cxx::__normal_iterator<T1, T2>
f49(const __gnu_cxx::__normal_iterator<T3, T2> &a0)
  requires(std::is_convertible_v<T3, T1>)
{
  return __gnu_cxx::__normal_iterator<T1, T2>(a0);
}

template <typename T1>
std::compare_three_way_result_t<T1> f50(const std::move_iterator<T1> &a0,
                                        const std::move_iterator<T1> &a1)
  requires(std::three_way_comparable<T1>)
{
  return std::operator<=> <T1>(a0, a1);
}

template <typename T1>
std::compare_three_way_result_t<T1, T1> f51(const std::reverse_iterator<T1> &a0,
                                            const std::reverse_iterator<T1> &a1)
  requires(std::three_way_comparable<T1>)
{
  return std::operator<=> <T1>(a0, a1);
}

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f52(const std::move_iterator<T1> &a0,
                                            const std::move_iterator<T2> &a1)
  requires(std::three_way_comparable_with<T2, T1>)
{
  return std::operator<=> <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f53(const std::reverse_iterator<T1> &a0,
                                            const std::reverse_iterator<T2> &a1)
  requires(std::three_way_comparable_with<T2, T1>)
{
  return std::operator<=> <T1, T2>(a0, a1);
}
