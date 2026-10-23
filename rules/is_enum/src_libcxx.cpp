#include <type_traits>
#include <utility>

template <typename T1>
typename std::is_enum<T1>::value_type f1(const std::is_enum<T1> &a0) {
  return static_cast<typename std::is_enum<T1>::value_type>(a0);
}
