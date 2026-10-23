#include <type_traits>
#include <utility>

template <typename T1> auto f1() -> decltype(std::declval<T1>()) {
  return std::declval<T1>();
}

using t1 = std::false_type;

using t2 = std::true_type;
