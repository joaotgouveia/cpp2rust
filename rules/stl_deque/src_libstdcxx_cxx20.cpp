#include <algorithm>
#include <deque>
#include <utility>

template <
    typename T1, typename T2, typename T3, typename T4,
    typename T5 = std::_Require<
        std::is_same<std::_Deque_iterator<T1, T2, T3>,
                     std::_Deque_iterator<T1, const T1 &,
                                          std::__ptr_rebind<T3, const T1>>>,
        std::is_same<
            T4, std::_Deque_iterator<T1, T1 &, std::__ptr_rebind<T3, T1>>>>>
std::_Deque_iterator<T1, T2, T3> f39(const T4 &a0)
  requires(!(std::is_same<T4, std::_Deque_iterator<T1, T2, T3>>::value))
{
  return std::_Deque_iterator<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::strong_ordering f40(const std::_Deque_iterator<T1, T2, T3> &a0,
                         const std::_Deque_iterator<T1, T2, T3> &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, typename T2>
auto f41(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return std::operator<=> <T1, T2>(a0, a1);
}
