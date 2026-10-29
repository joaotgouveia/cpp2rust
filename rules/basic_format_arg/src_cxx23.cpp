#include <format>
#include <utility>

template <typename T1> using t1 = std::basic_format_arg<T1>;

template <typename T1> using t2 = typename std::basic_format_arg<T1>::handle;

template <typename T1> std::basic_format_arg<T1> f1() {
  return std::basic_format_arg<T1>();
}

template <typename T1> bool f2(const std::basic_format_arg<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1>
typename std::basic_format_arg<T1>::handle
f3(const typename std::basic_format_arg<T1>::handle &a0) {
  return typename std::basic_format_arg<T1>::handle(a0);
}

template <typename T1>
typename std::basic_format_arg<T1>::handle &
f4(typename std::basic_format_arg<T1>::handle &a0,
   const typename std::basic_format_arg<T1>::handle &a1) {
  return a0.operator=(a1);
}

template <typename T1>
void f5(const typename std::basic_format_arg<T1>::handle &a0,
        std::basic_format_parse_context<typename T1::char_type> &a1, T1 &a2) {
  return a0.format(a1, a2);
}
