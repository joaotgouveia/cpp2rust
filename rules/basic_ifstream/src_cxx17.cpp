#include "../common/init.h"
#include "../common/traits.h"
#include <fstream>
#include <ios>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_ifstream<T1, T2>
f84(const T3 &a0, typename std::basic_ifstream<T1, T2>::openmode a1) {
  return std::basic_ifstream<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
void f85(std::basic_ifstream<T1, T2> &a0, const T3 &a1,
         typename std::basic_ifstream<T1, T2>::openmode a2) {
  return a0.template open<T3>(a1, a2);
}
