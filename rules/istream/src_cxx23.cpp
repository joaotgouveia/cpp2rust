#include <istream>
#include <utility>

template <typename T1>
std::basic_istream<char, T1> &f1(std::basic_istream<char, T1> &a0,
                                 unsigned char &a1) {
  return std::operator>>(a0, a1);
}

template <typename T1>
std::basic_istream<char, T1> &f2(std::basic_istream<char, T1> &a0,
                                 signed char &a1) {
  return std::operator>>(a0, a1);
}

template <typename T1, std::size_t T2>
std::basic_istream<char, T1> &f3(std::basic_istream<char, T1> &a0,
                                 unsigned char (&a1)[T2]) {
  return std::operator>>(a0, a1);
}

template <typename T1, std::size_t T2>
std::basic_istream<char, T1> &f4(std::basic_istream<char, T1> &a0,
                                 signed char (&a1)[T2]) {
  return std::operator>>(a0, a1);
}

template <typename T1, typename T2> T1 &&f5(T1 &&a0, T2 &&a1) {
  return std::operator>>(std::forward<T1>(a0), std::forward<T2>(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f6(std::basic_istream<T1, T2> &a0, T1 &a1) {
  return std::operator>>(a0, a1);
}

template <typename T1, typename T2, std::size_t T3>
std::basic_istream<T1, T2> &f7(std::basic_istream<T1, T2> &a0, T1 (&a1)[T3]) {
  return std::operator>>(a0, a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f8(std::basic_istream<T1, T2> &a0) {
  return std::ws(a0);
}
