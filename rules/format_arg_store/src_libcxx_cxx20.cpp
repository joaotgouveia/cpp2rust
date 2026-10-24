#include <format>
#include <utility>

template <typename T1, typename... T2>
using t1 = std::__format_arg_store<T1, T2...>;

template <typename T1, typename... T2>
std::__format_arg_store<T1, T2...> f1(T2 &...a0) {
  return std::__format_arg_store<T1, T2...>(a0...);
}
