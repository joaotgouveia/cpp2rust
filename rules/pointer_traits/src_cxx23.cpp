#include <algorithm>
#include <utility>

template <typename T1> using t1 = std::pointer_traits<T1 *>;

template <typename T1> using t2 = typename std::pointer_traits<T1 *>::pointer;

template <typename T1>
using t3 = typename std::pointer_traits<T1 *>::element_type;

template <typename T1>
using t4 = typename std::pointer_traits<T1 *>::difference_type;

template <typename T1> T1 *f1(T1 &a0) {
  return std::pointer_traits<T1 *>::pointer_to(a0);
}

template <typename T1>
T1 *f2(T1 &a0)
  requires(std::is_same<T1 *, T1 *>::value) &&
          (std::is_void<T1>::value == false)
{
  return std::pointer_traits<T1 *>::pointer_to(a0);
}

template <typename T1> using t5 = std::pointer_traits<T1>;

template <typename T1>
T1 f3(typename std::pointer_traits<T1>::element_type &a0) {
  return std::pointer_traits<T1>::pointer_to(a0);
}

template <typename T1>
auto f4(typename std::pointer_traits<T1>::element_type &a0) {
  return std::pointer_traits<T1>::pointer_to(a0);
}
