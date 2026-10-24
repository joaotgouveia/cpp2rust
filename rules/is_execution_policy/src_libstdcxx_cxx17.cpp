#include <algorithm>
#include <utility>

template <typename T1> using t1 = __pstl::execution::is_execution_policy<T1>;

template <typename T1>
bool f1(const __pstl::execution::is_execution_policy<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1>
bool f2(const __pstl::execution::is_execution_policy<T1> &a0) {
  return a0.operator()();
}
