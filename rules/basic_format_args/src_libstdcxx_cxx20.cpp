#include <algorithm>
#include <format>
#include <utility>

template <typename T1, typename... T2>
std::basic_format_args<T1> f2(const std::__format::_Arg_store<T1, T2...> &a0) {
  return std::basic_format_args<T1>(a0);
}
