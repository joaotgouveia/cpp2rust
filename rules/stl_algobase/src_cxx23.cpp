#include <algorithm>
#include <utility>

template <typename T1, typename T2> T2 f1(T1 a0, T1 a1, T2 a2) {
  return std::copy<T1, T2>(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2> T2 f2(T1 a0, T1 a1, T2 a2) {
  return std::copy_backward<T1, T2>(std::move(a0), std::move(a1),
                                    std::move(a2));
}

template <typename T1, typename T2> bool f3(T1 a0, T1 a1, T2 a2, T2 a3) {
  return std::equal<T1, T2>(std::move(a0), std::move(a1), std::move(a2),
                            std::move(a3));
}

template <typename T1, typename T2> bool f4(T1 a0, T1 a1, T2 a2) {
  return std::equal<T1, T2>(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
bool f5(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::equal<T1, T2, T3>(std::move(a0), std::move(a1), std::move(a2),
                                std::move(a3), std::move(a4));
}

template <typename T1, typename T2, typename T3>
bool f6(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::equal<T1, T2, T3>(std::move(a0), std::move(a1), std::move(a2),
                                std::move(a3));
}

template <typename T1, typename T2> void f7(T1 a0, T1 a1, const T2 &a2) {
  return std::fill<T1, T2>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
T1 f8(T1 a0, T2 a1, const T3 &a2) {
  return std::fill_n<T1, T2, T3>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2> bool f9(T1 a0, T1 a1, T2 a2) {
  return std::is_permutation<T1, T2>(std::move(a0), std::move(a1),
                                     std::move(a2));
}

template <typename T1, typename T2> void f10(T1 a0, T2 a1) {
  return std::iter_swap<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> bool f11(T1 a0, T1 a1, T2 a2, T2 a3) {
  return std::lexicographical_compare<T1, T2>(std::move(a0), std::move(a1),
                                              std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
bool f12(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::lexicographical_compare<T1, T2, T3>(std::move(a0), std::move(a1),
                                                  std::move(a2), std::move(a3),
                                                  std::move(a4));
}

template <typename T1, typename T2> auto f13(T1 a0, T1 a1, T2 a2, T2 a3) {
  return std::lexicographical_compare_three_way<T1, T2>(
      std::move(a0), std::move(a1), std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
auto f14(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::lexicographical_compare_three_way<T1, T2, T3>(
      std::move(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4));
}

template <typename T1, typename T2> T1 f15(T1 a0, T1 a1, const T2 &a2) {
  return std::lower_bound<T1, T2>(std::move(a0), std::move(a1), a2);
}

template <typename T1> const T1 &f16(const T1 &a0, const T1 &a1) {
  return std::max<T1>(a0, a1);
}

template <typename T1, typename T2>
const T1 &f17(const T1 &a0, const T1 &a1, T2 a2) {
  return std::max<T1, T2>(a0, a1, std::move(a2));
}

template <typename T1> const T1 &f18(const T1 &a0, const T1 &a1) {
  return std::min<T1>(a0, a1);
}

template <typename T1, typename T2>
const T1 &f19(const T1 &a0, const T1 &a1, T2 a2) {
  return std::min<T1, T2>(a0, a1, std::move(a2));
}

template <typename T1, typename T2> std::pair<T1, T2> f20(T1 a0, T1 a1, T2 a2) {
  return std::mismatch<T1, T2>(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2>
std::pair<T1, T2> f21(T1 a0, T1 a1, T2 a2, T2 a3) {
  return std::mismatch<T1, T2>(std::move(a0), std::move(a1), std::move(a2),
                               std::move(a3));
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T2> f22(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::mismatch<T1, T2, T3>(std::move(a0), std::move(a1), std::move(a2),
                                   std::move(a3), std::move(a4));
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T2> f23(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::mismatch<T1, T2, T3>(std::move(a0), std::move(a1), std::move(a2),
                                   std::move(a3));
}

template <typename T1, typename T2> T2 f24(T1 a0, T1 a1, T2 a2) {
  return std::move<T1, T2>(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2> T2 f25(T1 a0, T1 a1, T2 a2) {
  return std::move_backward<T1, T2>(std::move(a0), std::move(a1),
                                    std::move(a2));
}

template <typename T1, typename T2, typename T3>
T1 f26(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::search<T1, T2, T3>(std::move(a0), std::move(a1), std::move(a2),
                                 std::move(a3), std::move(a4));
}

template <typename T1, typename T2> T2 f27(T1 a0, T1 a1, T2 a2) {
  return std::swap_ranges<T1, T2>(std::move(a0), std::move(a1), std::move(a2));
}
