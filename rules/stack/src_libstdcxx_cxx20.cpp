#include <stack>
#include <utility>

template <typename T1, typename T2, typename T3,
          typename T4 =
              typename std::enable_if<std::uses_allocator<T2, T3>::value>::type>
std::stack<T1, T2> f21(const T3 &a0)
  requires(!(std::is_same<T3, T2>::value))
{
  return std::stack<T1, T2>(a0);
}
