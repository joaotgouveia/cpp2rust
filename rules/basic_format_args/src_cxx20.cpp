#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <format>
#include <utility>

template <typename T1> using t1 = std::basic_format_args<T1>;

template <typename T1>
std::basic_format_arg<T1> f1(const std::basic_format_args<T1> &a0,
                             std::size_t a1) {
  return a0.get(a1);
}
