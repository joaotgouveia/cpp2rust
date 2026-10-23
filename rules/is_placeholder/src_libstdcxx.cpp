#include <functional>
#include <utility>

template <typename T1> int f1(const std::is_placeholder<T1> &a0) {
  return static_cast<int>(a0);
}
