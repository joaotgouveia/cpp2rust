#include "../common/init.h"
#include "../common/traits.h"
#include <system_error>
#include <utility>

template <typename T1> using t1 = std::is_error_condition_enum<T1>;

template <typename T1>
typename std::is_error_condition_enum<T1>::value_type
f1(const std::is_error_condition_enum<T1> &a0) {
  return static_cast<typename std::is_error_condition_enum<T1>::value_type>(a0);
}
