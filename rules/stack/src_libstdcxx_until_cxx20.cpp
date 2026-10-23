#include <stack>
#include <utility>

template <
    typename T1, typename T2, typename T3,
    typename T4 =
        typename std::enable_if<std::uses_allocator<T2, T3>::value>::type,
    typename std::enable_if<(!(std::is_same<T3, T2>::value)), int>::type T5 = 0>
std::stack<T1, T2> f20(const T3 &a0) {
  return std::stack<T1, T2>(a0);
}
