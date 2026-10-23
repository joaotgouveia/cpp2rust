#include <algorithm>
#include <codecvt>
#include <utility>

template <typename T1, unsigned long T2, std::codecvt_mode T3>
typename std::codecvt_utf8_utf16<T1, T2, T3>::result
f2(const std::codecvt_utf8_utf16<T1, T2, T3> &a0, std::mbstate_t &a1,
   T1 const *a2, T1 const *a3, T1 const *&a4, char *a5, char *a6, char *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

template <typename T1, unsigned long T2, std::codecvt_mode T3>
typename std::codecvt_utf8_utf16<T1, T2, T3>::result
f3(const std::codecvt_utf8_utf16<T1, T2, T3> &a0, std::mbstate_t &a1, char *a2,
   char *a3, char *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

template <typename T1, unsigned long T2, std::codecvt_mode T3>
typename std::codecvt_utf8_utf16<T1, T2, T3>::result
f4(const std::codecvt_utf8_utf16<T1, T2, T3> &a0, std::mbstate_t &a1,
   char const *a2, char const *a3, char const *&a4, T1 *a5, T1 *a6, T1 *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

template <typename T1, unsigned long T2, std::codecvt_mode T3>
int f5(const std::codecvt_utf8_utf16<T1, T2, T3> &a0) {
  return a0.encoding();
}

template <typename T1, unsigned long T2, std::codecvt_mode T3>
bool f6(const std::codecvt_utf8_utf16<T1, T2, T3> &a0) {
  return a0.always_noconv();
}

template <typename T1, unsigned long T2, std::codecvt_mode T3>
int f7(const std::codecvt_utf8_utf16<T1, T2, T3> &a0, std::mbstate_t &a1,
       char const *a2, char const *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

template <typename T1, unsigned long T2, std::codecvt_mode T3>
int f8(const std::codecvt_utf8_utf16<T1, T2, T3> &a0) {
  return a0.max_length();
}
