#include <type_traits>
#include <utility>

template <typename T1, typename T2>
using t1 = std::is_layout_compatible<T1, T2>;

template <typename T1, typename T2>
bool f1(const std::is_layout_compatible<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f2(const std::is_layout_compatible<T1, T2> &a0) {
  return a0.operator()();
}

template <typename T1, typename T2> bool f3() {
  return std::is_layout_compatible<T1, T2>::value;
}
