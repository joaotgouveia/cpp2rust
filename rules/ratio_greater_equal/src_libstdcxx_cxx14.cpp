#include <ratio>
#include <utility>

template <typename T1, typename T2>
bool f2(const std::ratio_greater_equal<T1, T2> &a0) {
  return a0.operator()();
}
