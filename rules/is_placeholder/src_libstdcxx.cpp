#include <functional>
#include <utility>

template <typename T1>
typename std::is_placeholder<T1>::value_type
f1(const std::is_placeholder<T1> &a0) {
  return static_cast<typename std::is_placeholder<T1>::value_type>(a0);
}
