#include <cstddef>
#include <format>
#include <utility>

template <typename T1, typename... T2>
std::basic_format_args<T1> f3(const std::__format_arg_store<T1, T2...> &a0) {
  return std::basic_format_args<T1>(a0);
}
