#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <cstddef>
#include <initializer_list>
#include <ios>
#include <iosfwd>
#include <locale>
#include <memory>
#include <string>
#include <string_view>
#include <utility>

template <typename T1>
bool f10(const typename std::messages_byname<T1>::string_type &a0,
         std::basic_string_view<T1, std::char_traits<T1>> a1) {
  return a0.starts_with(a1);
}

template <typename T1>
bool f11(const typename std::messages_byname<T1>::string_type &a0,
         std::basic_string_view<T1, std::char_traits<T1>> a1) {
  return a0.ends_with(a1);
}
