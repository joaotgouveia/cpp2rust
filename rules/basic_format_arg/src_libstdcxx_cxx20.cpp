#include <format>
#include <utility>

template <typename T1>
typename std::basic_format_arg<T1>::handle
f4(const typename std::basic_format_arg<T1>::handle &a0) {
  return typename std::basic_format_arg<T1>::handle(a0);
}

template <typename T1>
typename std::basic_format_arg<T1>::handle &
f5(typename std::basic_format_arg<T1>::handle &a0,
   const typename std::basic_format_arg<T1>::handle &a1) {
  return a0.operator=(a1);
}
