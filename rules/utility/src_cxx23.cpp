#include <algorithm>
#include <utility>

using t1 = std::_Swallow_assign;

std::_Swallow_assign f1() { return std::_Swallow_assign(); }

std::_Swallow_assign f2(const std::_Swallow_assign &a0) {
  return std::_Swallow_assign(a0);
}

std::_Swallow_assign f3(std::_Swallow_assign &&a0) {
  return std::_Swallow_assign(std::move(a0));
}

std::_Swallow_assign &f4(std::_Swallow_assign &a0,
                         const std::_Swallow_assign &a1) {
  return a0.operator=(a1);
}

std::_Swallow_assign &f5(std::_Swallow_assign &a0, std::_Swallow_assign &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
const std::_Swallow_assign &f6(const std::_Swallow_assign &a0, const T1 &a1) {
  return a0.template operator= <T1>(a1);
}

template <typename T1> std::add_const_t<T1> &f7(T1 &a0) {
  return std::as_const(a0);
}

template <typename T1, typename T2> bool f8(T1 a0, T2 a1) {
  return std::cmp_equal(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> bool f9(T1 a0, T2 a1) {
  return std::cmp_greater(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> bool f10(T1 a0, T2 a1) {
  return std::cmp_greater_equal(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> bool f11(T1 a0, T2 a1) {
  return std::cmp_less(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> bool f12(T1 a0, T2 a1) {
  return std::cmp_less_equal(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> bool f13(T1 a0, T2 a1) {
  return std::cmp_not_equal(std::move(a0), std::move(a1));
}

template <typename T1, typename T2 = T1> T1 f14(T1 &a0, T2 &&a1) {
  return std::exchange(a0, std::forward<T2>(a1));
}

const std::_Swallow_assign &f15() { return std::ignore; }

const std::in_place_t &f16() { return std::in_place; }

template <std::size_t T1> std::in_place_index_t<T1> f17() {
  return std::in_place_index<T1>;
}

using t2 = std::in_place_t;

std::in_place_t f18() { return std::in_place_t(); }

std::in_place_t f19(const std::in_place_t &a0) { return std::in_place_t(a0); }

std::in_place_t f20(std::in_place_t &&a0) {
  return std::in_place_t(std::move(a0));
}

std::in_place_t &f21(std::in_place_t &a0, const std::in_place_t &a1) {
  return a0.operator=(a1);
}

std::in_place_t &f22(std::in_place_t &a0, std::in_place_t &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> std::in_place_type_t<T1> f23() {
  return std::in_place_type<T1>;
}

template <typename T1, typename T2> bool f24(T2 a0) {
  return std::in_range<T1>(std::move(a0));
}

template <std::size_t... T1> using t3 = std::index_sequence<T1...>;

template <typename... T1> using t4 = std::index_sequence_for<T1...>;

template <std::size_t T1> using t5 = std::make_index_sequence<T1>;

template <typename T1, T1 T2> using t6 = std::make_integer_sequence<T1, T2>;

const std::sorted_equivalent_t &f25() { return std::sorted_equivalent; }

using t7 = std::sorted_equivalent_t;

std::sorted_equivalent_t f26() { return std::sorted_equivalent_t(); }

std::sorted_equivalent_t f27(const std::sorted_equivalent_t &a0) {
  return std::sorted_equivalent_t(a0);
}

std::sorted_equivalent_t f28(std::sorted_equivalent_t &&a0) {
  return std::sorted_equivalent_t(std::move(a0));
}

std::sorted_equivalent_t &f29(std::sorted_equivalent_t &a0,
                              const std::sorted_equivalent_t &a1) {
  return a0.operator=(a1);
}

std::sorted_equivalent_t &f30(std::sorted_equivalent_t &a0,
                              std::sorted_equivalent_t &&a1) {
  return a0.operator=(std::move(a1));
}

const std::sorted_unique_t &f31() { return std::sorted_unique; }

using t8 = std::sorted_unique_t;

std::sorted_unique_t f32() { return std::sorted_unique_t(); }

std::sorted_unique_t f33(const std::sorted_unique_t &a0) {
  return std::sorted_unique_t(a0);
}

std::sorted_unique_t f34(std::sorted_unique_t &&a0) {
  return std::sorted_unique_t(std::move(a0));
}

std::sorted_unique_t &f35(std::sorted_unique_t &a0,
                          const std::sorted_unique_t &a1) {
  return a0.operator=(a1);
}

std::sorted_unique_t &f36(std::sorted_unique_t &a0, std::sorted_unique_t &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> std::underlying_type_t<T1> f37(T1 a0) {
  return std::to_underlying(std::move(a0));
}

template <std::size_t T1, typename T2> using t9 = std::tuple_element_t<T1, T2>;

template <typename T1> std::size_t f38() { return std::tuple_size_v<T1>; }

void f39() { return std::unreachable(); }
