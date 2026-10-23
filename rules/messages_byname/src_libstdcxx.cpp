#include <algorithm>
#include <ios>
#include <locale>
#include <string>
#include <utility>

template <typename T1> using t3 = typename std::messages_byname<T1>::char_type;

template <typename T1>
int f3(const std::messages_byname<T1> &a0, const std::basic_string<char> &a1,
       const std::locale &a2) {
  return a0.open(a1, a2);
}

template <typename T1>
int f4(const std::messages_byname<T1> &a0, const std::basic_string<char> &a1,
       const std::locale &a2, const char *a3) {
  return a0.open(a1, a2, a3);
}

template <typename T1>
std::basic_string<T1> f5(const std::messages_byname<T1> &a0, int a1, int a2,
                         int a3, const std::basic_string<T1> &a4) {
  return a0.get(a1, a2, a3, a4);
}

template <typename T1> void f6(const std::messages_byname<T1> &a0, int a1) {
  return a0.close(a1);
}
