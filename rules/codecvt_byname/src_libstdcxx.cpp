#include <algorithm>
#include <codecvt>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::codecvt_byname<T1, T2, T3> f1(const char *a0, std::size_t a1) {
  return std::codecvt_byname<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::codecvt_byname<T1, T2, T3> f2(const std::string &a0, std::size_t a1) {
  return std::codecvt_byname<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::codecvt_byname<T1, T2, T3>::result
f3(const std::codecvt_byname<T1, T2, T3> &a0, T3 &a1, T1 const *a2,
   T1 const *a3, T1 const *&a4, T2 *a5, T2 *a6, T2 *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

template <typename T1, typename T2, typename T3>
typename std::codecvt_byname<T1, T2, T3>::result
f4(const std::codecvt_byname<T1, T2, T3> &a0, T3 &a1, T2 *a2, T2 *a3, T2 *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
typename std::codecvt_byname<T1, T2, T3>::result
f5(const std::codecvt_byname<T1, T2, T3> &a0, T3 &a1, T2 const *a2,
   T2 const *a3, T2 const *&a4, T1 *a5, T1 *a6, T1 *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

template <typename T1, typename T2, typename T3>
int f6(const std::codecvt_byname<T1, T2, T3> &a0) {
  return a0.encoding();
}

template <typename T1, typename T2, typename T3>
bool f7(const std::codecvt_byname<T1, T2, T3> &a0) {
  return a0.always_noconv();
}

template <typename T1, typename T2, typename T3>
int f8(const std::codecvt_byname<T1, T2, T3> &a0, T3 &a1, T2 const *a2,
       T2 const *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
int f9(const std::codecvt_byname<T1, T2, T3> &a0) {
  return a0.max_length();
}
