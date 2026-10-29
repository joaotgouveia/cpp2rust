#include <chrono>
#include <utility>

template <typename T1> using t1 = std::chrono::duration_values<T1>;

template <typename T1> T1 f1() {
  return std::chrono::duration_values<T1>::zero();
}

template <typename T1> T1 f2() {
  return std::chrono::duration_values<T1>::max();
}

template <typename T1> T1 f3() {
  return std::chrono::duration_values<T1>::min();
}
