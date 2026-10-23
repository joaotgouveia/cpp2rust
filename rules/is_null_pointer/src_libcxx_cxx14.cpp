#include <type_traits>
#include <utility>

template <typename T1> using t2 = std::is_null_pointer<T1>;

template <typename T1>
typename std::is_null_pointer<T1>::value_type
f3(const std::is_null_pointer<T1> &a0) {
  return static_cast<typename std::is_null_pointer<T1>::value_type>(a0);
}
