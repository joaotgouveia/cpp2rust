#include <algorithm>
#include <bitset>
#include <utility>

template <std::size_t T1>
std::basic_string<char, std::char_traits<char>, std::allocator<char>>
f37(const std::bitset<T1> &a0) {
  return a0.to_string();
}

template <std::size_t T1, typename T2, typename T3, typename T4>
std::bitset<T1> f38(const std::basic_string<T2, T3, T4> &a0, std::size_t a1) {
  return std::bitset<T1>(a0, a1);
}

template <std::size_t T1, typename T2, typename T3, typename T4>
std::bitset<T1> f39(const std::basic_string<T2, T3, T4> &a0, std::size_t a1,
                    std::size_t a2) {
  return std::bitset<T1>(a0, a1, a2);
}

template <std::size_t T1, typename T2, typename T3, typename T4>
std::bitset<T1> f40(const std::basic_string<T2, T3, T4> &a0, std::size_t a1,
                    std::size_t a2, T2 a3, T2 a4) {
  return std::bitset<T1>(a0, a1, a2, a3, a4);
}

template <std::size_t T1, typename T2>
std::bitset<T1> f41(const T2 *a0, typename std::basic_string<T2>::size_type a1,
                    T2 a2, T2 a3) {
  return std::bitset<T1>(a0, a1, a2, a3);
}

template <std::size_t T1, typename T2, typename T3, typename T4>
std::basic_string<T2, T3, T4> f42(const std::bitset<T1> &a0) {
  return a0.template to_string<T2, T3, T4>();
}

template <std::size_t T1, typename T2, typename T3>
std::basic_string<T2, T3, std::allocator<T2>> f43(const std::bitset<T1> &a0) {
  return a0.template to_string<T2, T3>();
}

template <std::size_t T1, typename T2>
std::basic_string<T2, std::char_traits<T2>, std::allocator<T2>>
f44(const std::bitset<T1> &a0) {
  return a0.template to_string<T2>();
}

template <typename T1, typename T2, std::size_t T3>
std::basic_ostream<T1, T2> &f45(std::basic_ostream<T1, T2> &a0,
                                const std::bitset<T3> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, std::size_t T3>
std::basic_istream<T1, T2> &f46(std::basic_istream<T1, T2> &a0,
                                std::bitset<T3> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}
