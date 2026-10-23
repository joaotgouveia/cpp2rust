#include <algorithm>
#include <codecvt>
#include <utility>

template <typename T1, unsigned long T2, std::codecvt_mode T3>
typename std::codecvt_utf8<T1, T2, T3>::result
f2(const std::codecvt_utf8<T1, T2, T3> &a0,
   typename std::codecvt_utf8<T1, T2, T3>::state_type &a1,
   const typename std::codecvt_utf8<T1, T2, T3>::intern_type *a2,
   const typename std::codecvt_utf8<T1, T2, T3>::intern_type *a3,
   const typename std::codecvt_utf8<T1, T2, T3>::intern_type *&a4,
   typename std::codecvt_utf8<T1, T2, T3>::extern_type *a5,
   typename std::codecvt_utf8<T1, T2, T3>::extern_type *a6,
   typename std::codecvt_utf8<T1, T2, T3>::extern_type *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

template <typename T1, unsigned long T2, std::codecvt_mode T3>
typename std::codecvt_utf8<T1, T2, T3>::result
f3(const std::codecvt_utf8<T1, T2, T3> &a0,
   typename std::codecvt_utf8<T1, T2, T3>::state_type &a1,
   typename std::codecvt_utf8<T1, T2, T3>::extern_type *a2,
   typename std::codecvt_utf8<T1, T2, T3>::extern_type *a3,
   typename std::codecvt_utf8<T1, T2, T3>::extern_type *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

template <typename T1, unsigned long T2, std::codecvt_mode T3>
typename std::codecvt_utf8<T1, T2, T3>::result
f4(const std::codecvt_utf8<T1, T2, T3> &a0,
   typename std::codecvt_utf8<T1, T2, T3>::state_type &a1,
   const typename std::codecvt_utf8<T1, T2, T3>::extern_type *a2,
   const typename std::codecvt_utf8<T1, T2, T3>::extern_type *a3,
   const typename std::codecvt_utf8<T1, T2, T3>::extern_type *&a4,
   typename std::codecvt_utf8<T1, T2, T3>::intern_type *a5,
   typename std::codecvt_utf8<T1, T2, T3>::intern_type *a6,
   typename std::codecvt_utf8<T1, T2, T3>::intern_type *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

template <typename T1, unsigned long T2, std::codecvt_mode T3>
int f5(const std::codecvt_utf8<T1, T2, T3> &a0) {
  return a0.encoding();
}

template <typename T1, unsigned long T2, std::codecvt_mode T3>
bool f6(const std::codecvt_utf8<T1, T2, T3> &a0) {
  return a0.always_noconv();
}

template <typename T1, unsigned long T2, std::codecvt_mode T3>
int f7(const std::codecvt_utf8<T1, T2, T3> &a0,
       typename std::codecvt_utf8<T1, T2, T3>::state_type &a1,
       const typename std::codecvt_utf8<T1, T2, T3>::extern_type *a2,
       const typename std::codecvt_utf8<T1, T2, T3>::extern_type *a3,
       std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

template <typename T1, unsigned long T2, std::codecvt_mode T3>
int f8(const std::codecvt_utf8<T1, T2, T3> &a0) {
  return a0.max_length();
}
