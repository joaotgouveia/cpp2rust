#include <chrono>
#include <utility>

template <typename T1>
bool f2(const std::chrono::treat_as_floating_point<T1> &a0) {
  return a0.operator()();
}
