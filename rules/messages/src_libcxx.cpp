#include <cstddef>
#include <ios>
#include <locale>
#include <string>
#include <utility>

template <typename T1> std::messages<T1> f6(std::size_t a0) {
  return std::messages<T1>(a0);
}

template <typename T1>
std::intptr_t f7(const std::messages<T1> &a0, const std::basic_string<char> &a1,
                 const std::locale &a2) {
  return a0.open(a1, a2);
}

template <typename T1>
std::basic_string<T1> f8(const std::messages<T1> &a0, std::intptr_t a1, int a2,
                         int a3, const std::basic_string<T1> &a4) {
  return a0.get(a1, a2, a3, a4);
}

template <typename T1> void f9(const std::messages<T1> &a0, std::intptr_t a1) {
  return a0.close(a1);
}

template <typename T1> long f10(const std::messages<T1> &a0) {
  return a0.use_count();
}

using t4 = std::messages_base;

using t5 = std::messages_base::catalog;

std::messages_base f11() { return std::messages_base(); }

std::messages_base f12(const std::messages_base &a0) {
  return std::messages_base(a0);
}

std::messages_base f13(std::messages_base &&a0) {
  return std::messages_base(std::move(a0));
}

std::messages_base &f14(std::messages_base &a0, const std::messages_base &a1) {
  return a0.operator=(a1);
}

std::messages_base &f15(std::messages_base &a0, std::messages_base &&a1) {
  return a0.operator=(std::move(a1));
}
