#include <utility>
#include <valarray>

template <typename T1>
const std::gslice_array<T1> &f26(const std::gslice_array<T1> &a0,
                                 const std::gslice_array<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f27(const std::gslice_array<T1> &a0, const T2 &a1) {
  return a0.template operator*= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f28(const std::gslice_array<T1> &a0, const T2 &a1) {
  return a0.template operator/= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f29(const std::gslice_array<T1> &a0, const T2 &a1) {
  return a0.template operator%= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f30(const std::gslice_array<T1> &a0, const T2 &a1) {
  return a0.template operator+= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f31(const std::gslice_array<T1> &a0, const T2 &a1) {
  return a0.template operator-= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f32(const std::gslice_array<T1> &a0, const T2 &a1) {
  return a0.template operator^= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f33(const std::gslice_array<T1> &a0, const T2 &a1) {
  return a0.template operator&= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f34(const std::gslice_array<T1> &a0, const T2 &a1) {
  return a0.template operator|= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f35(const std::gslice_array<T1> &a0, const T2 &a1) {
  return a0.template operator<<= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f36(const std::gslice_array<T1> &a0, const T2 &a1) {
  return a0.template operator>>= <T2>(a1);
}
