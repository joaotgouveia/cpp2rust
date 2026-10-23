#include <functional>
#include <string>
#include <utility>

template <typename T1> using t1 = std::logical_not<T1>;

template <typename T1> bool f1(const std::logical_not<T1> &a0, const T1 &a1) {
  return a0.operator()(a1);
}
