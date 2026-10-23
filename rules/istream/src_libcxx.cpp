#include <bitset>
#include <iosfwd>
#include <istream>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f12(std::basic_istream<T1, T2> &&a0,
                                std::basic_string<T1, T2, T3> &a1) {
  return std::getline<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f13(std::basic_istream<T1, T2> &&a0,
                                std::basic_string<T1, T2, T3> &a1, T1 a2) {
  return std::getline<T1, T2, T3>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f14(std::basic_istream<T1, T2> &a0,
                                std::basic_string<T1, T2, T3> &a1) {
  return std::getline<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f15(std::basic_istream<T1, T2> &a0,
                                std::basic_string<T1, T2, T3> &a1, T1 a2) {
  return std::getline<T1, T2, T3>(a0, a1, a2);
}

using t1 = std::iostream;

using t2 = std::istream;

template <typename T1, typename T2, std::size_t T3>
std::basic_istream<T1, T2> &f16(std::basic_istream<T1, T2> &a0,
                                std::bitset<T3> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f17(std::basic_istream<T1, T2> &a0,
                                std::basic_string<T1, T2, T3> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

using t3 = std::wiostream;

using t4 = std::wistream;
