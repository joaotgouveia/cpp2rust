#include <cstddef>
#include <initializer_list>
#include <ios>
#include <locale>
#include <memory>
#include <string.h>
#include <string>
#include <string_view>
#include <utility>

template <typename T1> using t13 = typename std::messages_byname<T1>::catalog;

template <typename T1> long f86(const std::messages_byname<T1> &a0) {
  return a0.use_count();
}

template <typename T1>
std::basic_string_view<T1, std::char_traits<T1>>
f87(const typename std::messages_byname<T1>::string_type &a0) {
  return static_cast<std::basic_string_view<T1, std::char_traits<T1>>>(a0);
}
