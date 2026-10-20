#include "../common/init.h"
#include "../common/traits.h"
#include <spanstream>
#include <utility>

template <typename T1, typename T2>
void f1(std::basic_ispanstream<T1, T2> &a0,
        std::basic_ispanstream<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
void f2(std::basic_ospanstream<T1, T2> &a0,
        std::basic_ospanstream<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
void f3(std::basic_spanbuf<T1, T2> &a0, std::basic_spanbuf<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
void f4(std::basic_spanstream<T1, T2> &a0, std::basic_spanstream<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
