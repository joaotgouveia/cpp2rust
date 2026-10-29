#include <utility>
#include <valarray>

template <typename T1> using t1 = std::mask_array<T1>;

template <typename T1> using t2 = typename std::mask_array<T1>::value_type;

template <typename T1> std::mask_array<T1> f1(const std::mask_array<T1> &a0) {
  return std::mask_array<T1>(a0);
}

template <typename T1>
std::mask_array<T1> &f2(std::mask_array<T1> &a0,
                        const std::mask_array<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
void f3(const std::mask_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
void f4(const std::mask_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator*=(a1);
}

template <typename T1>
void f5(const std::mask_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator/=(a1);
}

template <typename T1>
void f6(const std::mask_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator%=(a1);
}

template <typename T1>
void f7(const std::mask_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator+=(a1);
}

template <typename T1>
void f8(const std::mask_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator-=(a1);
}

template <typename T1>
void f9(const std::mask_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator^=(a1);
}

template <typename T1>
void f10(const std::mask_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator&=(a1);
}

template <typename T1>
void f11(const std::mask_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator|=(a1);
}

template <typename T1>
void f12(const std::mask_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator<<=(a1);
}

template <typename T1>
void f13(const std::mask_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator>>=(a1);
}

template <typename T1> void f14(const std::mask_array<T1> &a0, const T1 &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
void f15(const std::mask_array<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
void f16(const std::mask_array<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator*= <T2>(a1);
}

template <typename T1, typename T2>
void f17(const std::mask_array<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator/= <T2>(a1);
}

template <typename T1, typename T2>
void f18(const std::mask_array<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator%= <T2>(a1);
}

template <typename T1, typename T2>
void f19(const std::mask_array<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator+= <T2>(a1);
}

template <typename T1, typename T2>
void f20(const std::mask_array<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator-= <T2>(a1);
}

template <typename T1, typename T2>
void f21(const std::mask_array<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator^= <T2>(a1);
}

template <typename T1, typename T2>
void f22(const std::mask_array<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator&= <T2>(a1);
}

template <typename T1, typename T2>
void f23(const std::mask_array<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator|= <T2>(a1);
}

template <typename T1, typename T2>
void f24(const std::mask_array<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator<<= <T2>(a1);
}

template <typename T1, typename T2>
void f25(const std::mask_array<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator>>= <T2>(a1);
}
