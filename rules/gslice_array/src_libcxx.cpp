#include <utility>
#include <valarray>

template <typename T1>
const std::gslice_array<T1> &f15(const std::gslice_array<T1> &a0,
                                 const std::gslice_array<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f16(const std::gslice_array<T1> &a0, const T2 &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f17(const std::gslice_array<T1> &a0, const T2 &a1) {
  return a0.template operator*= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f18(const std::gslice_array<T1> &a0, const T2 &a1) {
  return a0.template operator/= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f19(const std::gslice_array<T1> &a0, const T2 &a1) {
  return a0.template operator%= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f20(const std::gslice_array<T1> &a0, const T2 &a1) {
  return a0.template operator+= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f21(const std::gslice_array<T1> &a0, const T2 &a1) {
  return a0.template operator-= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f22(const std::gslice_array<T1> &a0, const T2 &a1) {
  return a0.template operator^= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f23(const std::gslice_array<T1> &a0, const T2 &a1) {
  return a0.template operator&= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f24(const std::gslice_array<T1> &a0, const T2 &a1) {
  return a0.template operator|= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f25(const std::gslice_array<T1> &a0, const T2 &a1) {
  return a0.template operator<<= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f26(const std::gslice_array<T1> &a0, const T2 &a1) {
  return a0.template operator>>= <T2>(a1);
}
