#include <format>
#include <utility>

template <typename T1, typename T2>
decltype(auto) f1(T1 &&a0, std::basic_format_arg<T2> a1) {
  return std::visit_format_arg<T1, T2>(std::forward<T1>(a0), a1);
}
