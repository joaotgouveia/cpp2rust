#include <format>
#include <utility>

template <typename T1>
typename std::basic_format_arg<T1>::handle
f6(typename std::__basic_format_arg_value<T1>::__handle &a0) {
  return typename std::basic_format_arg<T1>::handle(a0);
}
