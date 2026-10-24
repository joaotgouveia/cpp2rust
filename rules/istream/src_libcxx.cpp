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
  return std::getline<T1, T2, T3>(std::move(a0), a1, std::move(a2));
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f14(std::basic_istream<T1, T2> &a0,
                                std::basic_string<T1, T2, T3> &a1) {
  return std::getline<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f15(std::basic_istream<T1, T2> &a0,
                                std::basic_string<T1, T2, T3> &a1, T1 a2) {
  return std::getline<T1, T2, T3>(a0, a1, std::move(a2));
}

using t1 = std::iostream;

using t2 = std::istream;

template <
    typename T1, typename T2,
    std::__enable_if_t<std::_And<std::is_base_of<std::ios_base, T1>,
                                 std::__is_istreamable<T1 &, T2 &&>>::value,
                       int>
        T3 = 0>
T1 &&f16(T1 &&a0, T2 &&a1) {
  return std::operator>> <T1, T2>(std::forward<T1>(a0), std::forward<T2>(a1));
}

template <typename T1, typename T2, std::size_t T3>
std::basic_istream<T1, T2> &f17(std::basic_istream<T1, T2> &a0,
                                std::bitset<T3> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f18(std::basic_istream<T1, T2> &a0,
                                std::basic_string<T1, T2, T3> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

using t3 = std::wiostream;

using t4 = std::wistream;
