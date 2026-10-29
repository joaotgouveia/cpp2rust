#include <functional>
#include <utility>

template <typename T1> using t1 = std::is_placeholder<T1>;

template <typename T1> int f1(const std::is_placeholder<T1> &a0) {
  return static_cast<int>(a0);
}

template <typename T1> int f2(const std::is_placeholder<T1> &a0) {
  return a0.operator()();
}

template <typename T1> int f3() { return std::is_placeholder<T1>::value; }
