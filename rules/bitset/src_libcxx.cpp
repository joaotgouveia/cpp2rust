#include <algorithm>
#include <bitset>
#include <utility>

template <typename T1> void f47(bool &a0, std::__bit_reference<T1> a1) {
  return std::swap<T1>(a0, a1);
}

template <typename T1> void f48(std::__bit_reference<T1> a0, bool &a1) {
  return std::swap<T1>(a0, a1);
}

template <typename T1>
void f49(std::__bit_reference<T1> a0, std::__bit_reference<T1> a1) {
  return std::swap<T1>(a0, a1);
}

template <typename T1, typename T2>
void f50(std::__bit_reference<T1> a0, std::__bit_reference<T2> a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1> using t3 = std::__bit_const_reference<T1>;

template <typename T1>
std::__bit_const_reference<T1> f51(const std::__bit_const_reference<T1> &a0) {
  return std::__bit_const_reference<T1>(a0);
}

template <typename T1> bool f52(const std::__bit_const_reference<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1>
std::__bit_iterator<T1, true, 0> f53(const std::__bit_const_reference<T1> &a0) {
  return a0.operator&();
}

template <typename T1, bool T2> using t4 = std::__bit_reference<T1, T2>;

template <typename T1, bool T2>
std::__bit_reference<T1, T2> f54(const std::__bit_reference<T1, T2> &a0) {
  return std::__bit_reference<T1, T2>(a0);
}

template <typename T1, bool T2>
bool f55(const std::__bit_reference<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, bool T2>
bool f56(const std::__bit_reference<T1, T2> &a0) {
  return a0.operator~();
}

template <typename T1, bool T2>
std::__bit_reference<T1, T2> &f57(std::__bit_reference<T1, T2> &a0, bool a1) {
  return a0.operator=(a1);
}

template <typename T1, bool T2>
std::__bit_reference<T1, T2> &f58(std::__bit_reference<T1, T2> &a0,
                                  const std::__bit_reference<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, bool T2> void f59(std::__bit_reference<T1, T2> &a0) {
  return a0.flip();
}

template <std::size_t T1, typename T2,
          std::__enable_if_t<std::_IsCharLikeType<T2>::value, int> T3 = 0>
std::bitset<T1> f60(const T2 *a0, std::size_t a1, T2 a2, T2 a3) {
  return std::bitset<T1>(a0, a1, a2, a3);
}

template <std::size_t T1, typename T2, typename T3, typename T4>
std::bitset<T1> f61(const std::basic_string<T2, T3, T4> &a0,
                    typename std::basic_string<T2, T3, T4>::size_type a1,
                    typename std::basic_string<T2, T3, T4>::size_type a2, T2 a3,
                    T2 a4) {
  return std::bitset<T1>(a0, a1, a2, a3, a4);
}
