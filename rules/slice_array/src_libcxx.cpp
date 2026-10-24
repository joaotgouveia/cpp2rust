#include <utility>
#include <valarray>

template <typename T1>
const std::slice_array<T1> &f36(const std::slice_array<T1> &a0,
                                const std::slice_array<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f37(const std::slice_array<T1> &a0, const T2 &a1) {
  return a0.template operator*= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f38(const std::slice_array<T1> &a0, const T2 &a1) {
  return a0.template operator/= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f39(const std::slice_array<T1> &a0, const T2 &a1) {
  return a0.template operator%= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f40(const std::slice_array<T1> &a0, const T2 &a1) {
  return a0.template operator+= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f41(const std::slice_array<T1> &a0, const T2 &a1) {
  return a0.template operator-= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f42(const std::slice_array<T1> &a0, const T2 &a1) {
  return a0.template operator^= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f43(const std::slice_array<T1> &a0, const T2 &a1) {
  return a0.template operator&= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f44(const std::slice_array<T1> &a0, const T2 &a1) {
  return a0.template operator|= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f45(const std::slice_array<T1> &a0, const T2 &a1) {
  return a0.template operator<<= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f46(const std::slice_array<T1> &a0, const T2 &a1) {
  return a0.template operator>>= <T2>(a1);
}
