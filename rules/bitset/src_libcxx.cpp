#include <algorithm>
#include <bitset>
#include <utility>

template <typename T1> void f53(bool &a0, std::__bit_reference<T1> a1) {
  return std::swap<T1>(a0, a1);
}

template <typename T1> void f54(std::__bit_reference<T1> a0, bool &a1) {
  return std::swap<T1>(a0, a1);
}

template <typename T1>
void f55(std::__bit_reference<T1> a0, std::__bit_reference<T1> a1) {
  return std::swap<T1>(a0, a1);
}

template <typename T1, typename T2>
void f56(std::__bit_reference<T1> a0, std::__bit_reference<T2> a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1> using t3 = std::__bit_const_reference<T1>;

template <typename T1>
std::__bit_const_reference<T1> f57(const std::__bit_const_reference<T1> &a0) {
  return std::__bit_const_reference<T1>(a0);
}

template <typename T1>
std::__bit_const_reference<T1> f58(const std::__bit_reference<T1> &a0) {
  return std::__bit_const_reference<T1>(a0);
}

template <typename T1> bool f59(const std::__bit_const_reference<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1>
std::__bit_iterator<T1, true> f60(const std::__bit_const_reference<T1> &a0) {
  return a0.operator&();
}

template <typename T1, bool T2> using t4 = std::__bit_reference<T1, T2>;

template <typename T1, bool T2>
std::__bit_reference<T1, T2> f61(const std::__bit_reference<T1, T2> &a0) {
  return std::__bit_reference<T1, T2>(a0);
}

template <typename T1, bool T2>
bool f62(const std::__bit_reference<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, bool T2>
bool f63(const std::__bit_reference<T1, T2> &a0) {
  return a0.operator~();
}

template <typename T1, bool T2>
std::__bit_reference<T1, T2> &f64(std::__bit_reference<T1, T2> &a0, bool a1) {
  return a0.operator=(a1);
}

template <typename T1, bool T2>
std::__bit_reference<T1, T2> &f65(std::__bit_reference<T1, T2> &a0,
                                  const std::__bit_reference<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, bool T2> void f66(std::__bit_reference<T1, T2> &a0) {
  return a0.flip();
}

template <std::size_t T1, typename T2,
          std::__enable_if_t<std::_IsCharLikeType<T2>::value, int> T3 = 0>
std::bitset<T1> f67(const T2 *a0, std::size_t a1, T2 a2, T2 a3) {
  return std::bitset<T1>(a0, a1, a2, a3);
}

template <std::size_t T1, typename T2, typename T3, typename T4>
std::bitset<T1> f68(const std::basic_string<T2, T3, T4> &a0,
                    typename std::basic_string<T2, T3, T4>::size_type a1,
                    typename std::basic_string<T2, T3, T4>::size_type a2, T2 a3,
                    T2 a4) {
  return std::bitset<T1>(a0, a1, a2, a3, a4);
}
