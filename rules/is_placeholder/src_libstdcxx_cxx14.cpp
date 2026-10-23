#include <functional>
#include <utility>

template <typename T1> int f2(const std::is_placeholder<T1> &a0) {
  return a0.operator()();
}
