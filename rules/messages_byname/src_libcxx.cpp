#include <cstddef>
#include <ios>
#include <locale>
#include <string.h>
#include <string>
#include <utility>

template <typename T1> using t4 = typename std::messages_byname<T1>::catalog;

template <typename T1>
std::intptr_t f7(const std::messages_byname<T1> &a0,
                 const std::basic_string<char> &a1, const std::locale &a2) {
  return a0.open(a1, a2);
}

template <typename T1>
std::basic_string<T1> f8(const std::messages_byname<T1> &a0, std::intptr_t a1,
                         int a2, int a3, const std::basic_string<T1> &a4) {
  return a0.get(a1, a2, a3, a4);
}

template <typename T1>
void f9(const std::messages_byname<T1> &a0, std::intptr_t a1) {
  return a0.close(a1);
}

template <typename T1> long f10(const std::messages_byname<T1> &a0) {
  return a0.use_count();
}
