#include <type_traits>
#include <utility>

template <typename T1> bool f1(const std::is_arithmetic<T1> &a0) {
  return static_cast<bool>(a0);
}
