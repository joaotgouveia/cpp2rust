#include <type_traits>
#include <utility>

template <typename T1> bool f2(const std::is_volatile<T1> &a0) {
  return a0.operator()();
}
