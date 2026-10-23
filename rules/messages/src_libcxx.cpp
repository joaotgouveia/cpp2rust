#include <cstddef>
#include <initializer_list>
#include <ios>
#include <locale>
#include <memory>
#include <string>
#include <string_view>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> std::messages<T1> f85(std::size_t a0) {
  return std::messages<T1>(a0);
}

template <typename T1> long f86(const std::messages<T1> &a0) {
  return a0.use_count();
}

template <typename T1>
std::basic_string_view<T1, std::char_traits<T1>>
f87(const typename std::messages<T1>::string_type &a0) {
  return static_cast<std::basic_string_view<T1, std::char_traits<T1>>>(a0);
}

using t13 = std::messages_base;

using t14 = std::messages_base::catalog;

std::messages_base f88() { return std::messages_base(); }

std::messages_base f89(const std::messages_base &a0) {
  return std::messages_base(a0);
}

std::messages_base f90(std::messages_base &&a0) {
  return std::messages_base(std::move(a0));
}

std::messages_base &f91(std::messages_base &a0, const std::messages_base &a1) {
  return a0.operator=(a1);
}

std::messages_base &f92(std::messages_base &a0, std::messages_base &&a1) {
  return a0.operator=(std::move(a1));
}
