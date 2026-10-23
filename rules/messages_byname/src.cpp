#include <algorithm>
#include <cstddef>
#include <ios>
#include <locale>
#include <string>
#include <utility>

template <typename T1> using t1 = std::messages_byname<T1>;

template <typename T1>
using t2 = typename std::messages_byname<T1>::string_type;

template <typename T1>
std::messages_byname<T1> f1(const char *a0, std::size_t a1) {
  return std::messages_byname<T1>(a0, a1);
}

template <typename T1>
std::messages_byname<T1> f2(const std::string &a0, std::size_t a1) {
  return std::messages_byname<T1>(a0, a1);
}
