#include <type_traits>
#include <utility>

template <typename T1> using t1 = std::rank<T1>;

template <typename T1>
typename std::rank<T1>::value_type f1(const std::rank<T1> &a0) {
  return static_cast<typename std::rank<T1>::value_type>(a0);
}
