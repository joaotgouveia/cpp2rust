#include <fstream>
#include <iosfwd>
#include <utility>

template <typename T1, typename T2>
void f1(std::basic_filebuf<T1, T2> &a0, std::basic_filebuf<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
void f2(std::basic_fstream<T1, T2> &a0, std::basic_fstream<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
void f3(std::basic_ifstream<T1, T2> &a0, std::basic_ifstream<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
void f4(std::basic_ofstream<T1, T2> &a0, std::basic_ofstream<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
