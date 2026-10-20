#include "../common/init.h"
#include "../common/traits.h"
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
