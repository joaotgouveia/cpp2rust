#include <initializer_list>
#include <utility>

template <typename T1>
typename std::initializer_list<T1>::size_type
f6(const std::initializer_list<T1> &a0) {
  return a0.size();
}
