#include <algorithm>
#include <format>
#include <utility>

template <typename T1> using t1 = std::basic_format_args<T1>;

template <typename T1>
std::basic_format_arg<T1> f1(const std::basic_format_args<T1> &a0,
                             std::size_t a1) {
  return a0.get(a1);
}

template <typename T1, typename... T2>
std::basic_format_args<T1> f2(const std::__format::_Arg_store<T1, T2...> &a0) {
  return std::basic_format_args<T1>(a0);
}
