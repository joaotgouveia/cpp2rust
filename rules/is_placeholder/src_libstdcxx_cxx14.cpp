#include <functional>
#include <utility>

template <typename T1>
typename std::is_placeholder<T1>::value_type
f2(const std::is_placeholder<T1> &a0) {
  return a0.operator()();
}
