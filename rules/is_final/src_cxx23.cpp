#include <type_traits>
#include <utility>

template <typename T1> using t1 = std::is_final<T1>;

template <typename T1> bool f1(const std::is_final<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> bool f2(const std::is_final<T1> &a0) {
  return a0.operator()();
}

template <typename T1> bool f3() { return std::is_final<T1>::value; }
