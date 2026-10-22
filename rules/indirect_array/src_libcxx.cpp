#include "../common/init.h"
#include "../common/traits.h"
#include <utility>
#include <valarray>

template <typename T1>
const std::indirect_array<T1> &f15(const std::indirect_array<T1> &a0,
                                   const std::indirect_array<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
void f16(const std::indirect_array<T1> &a0,
         const typename std::indirect_array<T1>::value_type &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
void f17(const std::indirect_array<T1> &a0, const T2 &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
void f18(const std::indirect_array<T1> &a0, const T2 &a1) {
  return a0.template operator*= <T2>(a1);
}

template <typename T1, typename T2>
void f19(const std::indirect_array<T1> &a0, const T2 &a1) {
  return a0.template operator/= <T2>(a1);
}

template <typename T1, typename T2>
void f20(const std::indirect_array<T1> &a0, const T2 &a1) {
  return a0.template operator%= <T2>(a1);
}

template <typename T1, typename T2>
void f21(const std::indirect_array<T1> &a0, const T2 &a1) {
  return a0.template operator+= <T2>(a1);
}

template <typename T1, typename T2>
void f22(const std::indirect_array<T1> &a0, const T2 &a1) {
  return a0.template operator-= <T2>(a1);
}

template <typename T1, typename T2>
void f23(const std::indirect_array<T1> &a0, const T2 &a1) {
  return a0.template operator^= <T2>(a1);
}

template <typename T1, typename T2>
void f24(const std::indirect_array<T1> &a0, const T2 &a1) {
  return a0.template operator&= <T2>(a1);
}

template <typename T1, typename T2>
void f25(const std::indirect_array<T1> &a0, const T2 &a1) {
  return a0.template operator|= <T2>(a1);
}

template <typename T1, typename T2>
void f26(const std::indirect_array<T1> &a0, const T2 &a1) {
  return a0.template operator<<= <T2>(a1);
}

template <typename T1, typename T2>
void f27(const std::indirect_array<T1> &a0, const T2 &a1) {
  return a0.template operator>>= <T2>(a1);
}
