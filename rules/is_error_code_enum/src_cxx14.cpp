#include "../common/init.h"
#include "../common/traits.h"
#include <system_error>
#include <utility>

template <typename T1>
typename std::is_error_code_enum<T1>::value_type
f2(const std::is_error_code_enum<T1> &a0) {
  return a0.operator()();
}
