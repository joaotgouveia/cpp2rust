#include <algorithm>
#include <type_traits>
#include <utility>

template <typename T1> using t1 = std::add_const_t<T1>;

template <typename T1> using t2 = std::add_cv_t<T1>;

template <typename T1> using t3 = std::add_lvalue_reference_t<T1>;

template <typename T1> using t4 = std::add_pointer_t<T1>;

template <typename T1> using t5 = std::add_rvalue_reference_t<T1>;

template <typename T1> using t6 = std::add_volatile_t<T1>;

template <std::size_t T1, std::size_t T2>
using t7 = std::aligned_storage_t<T1, T2>;

template <std::size_t T1, typename... T2>
using t8 = std::aligned_union_t<T1, T2...>;

template <typename T1> std::size_t f1() { return std::alignment_of_v<T1>; }

template <bool T1> using t9 = std::bool_constant<T1>;

template <typename... T1> using t10 = std::common_reference_t<T1...>;

template <typename... T1> using t11 = std::common_type_t<T1...>;

template <bool T1, typename T2, typename T3>
using t12 = std::conditional_t<T1, T2, T3>;

template <typename... T1> bool f2() { return std::conjunction_v<T1...>; }

template <typename T1> using t13 = std::decay_t<T1>;

template <typename T1> auto f3() { return std::declval<T1>(); }

template <typename... T1> bool f4() { return std::disjunction_v<T1...>; }

template <bool T1, typename T2> using t14 = std::enable_if_t<T1, T2>;

template <typename T1, unsigned int T2> std::size_t f5() {
  return std::extent_v<T1, T2>;
}

using t15 = std::false_type;

template <typename T1> bool f6() {
  return std::has_unique_object_representations_v<T1>;
}

template <typename T1> bool f7() { return std::has_virtual_destructor_v<T1>; }

template <typename T1, typename... T2>
using t16 = std::invoke_result_t<T1, T2...>;

template <typename T1> bool f8() { return std::is_abstract_v<T1>; }

template <typename T1> bool f9() { return std::is_aggregate_v<T1>; }

template <typename T1> bool f10() { return std::is_arithmetic_v<T1>; }

template <typename T1> bool f11() { return std::is_array_v<T1>; }

template <typename T1, typename T2> bool f12() {
  return std::is_assignable_v<T1, T2>;
}

template <typename T1, typename T2> bool f13() {
  return std::is_base_of_v<T1, T2>;
}

template <typename T1> bool f14() { return std::is_bounded_array_v<T1>; }

template <typename T1> bool f15() { return std::is_class_v<T1>; }

template <typename T1> bool f16() { return std::is_compound_v<T1>; }

template <typename T1> bool f17() { return std::is_const_v<T1>; }

bool f18() { return std::is_constant_evaluated(); }

template <typename T1, typename... T2> bool f19() {
  return std::is_constructible_v<T1, T2...>;
}

template <typename T1, typename T2> bool f20() {
  return std::is_convertible_v<T1, T2>;
}

template <typename T1> bool f21() { return std::is_copy_assignable_v<T1>; }

template <typename T1> bool f22() { return std::is_copy_constructible_v<T1>; }

template <typename T1> bool f23() {
  return std::is_default_constructible_v<T1>;
}

template <typename T1> bool f24() { return std::is_destructible_v<T1>; }

template <typename T1> bool f25() { return std::is_empty_v<T1>; }

template <typename T1> bool f26() { return std::is_enum_v<T1>; }

template <typename T1> bool f27() { return std::is_final_v<T1>; }

template <typename T1> bool f28() { return std::is_floating_point_v<T1>; }

template <typename T1> bool f29() { return std::is_function_v<T1>; }

template <typename T1> bool f30() { return std::is_fundamental_v<T1>; }

template <typename T1> bool f31() { return std::is_integral_v<T1>; }

template <typename T1, typename T2, typename... T3> bool f32() {
  return std::is_invocable_r_v<T1, T2, T3...>;
}

template <typename T1, typename... T2> bool f33() {
  return std::is_invocable_v<T1, T2...>;
}

template <typename T1, typename T2> bool f34() {
  return std::is_layout_compatible_v<T1, T2>;
}

template <typename T1> bool f35() { return std::is_literal_type_v<T1>; }

template <typename T1> bool f36() { return std::is_lvalue_reference_v<T1>; }

template <typename T1> bool f37() {
  return std::is_member_function_pointer_v<T1>;
}

template <typename T1> bool f38() {
  return std::is_member_object_pointer_v<T1>;
}

template <typename T1> bool f39() { return std::is_member_pointer_v<T1>; }

template <typename T1> bool f40() { return std::is_move_assignable_v<T1>; }

template <typename T1> bool f41() { return std::is_move_constructible_v<T1>; }

template <typename T1, typename T2> bool f42() {
  return std::is_nothrow_assignable_v<T1, T2>;
}

template <typename T1, typename... T2> bool f43() {
  return std::is_nothrow_constructible_v<T1, T2...>;
}

template <typename T1, typename T2> bool f44() {
  return std::is_nothrow_convertible_v<T1, T2>;
}

template <typename T1> bool f45() {
  return std::is_nothrow_copy_assignable_v<T1>;
}

template <typename T1> bool f46() {
  return std::is_nothrow_copy_constructible_v<T1>;
}

template <typename T1> bool f47() {
  return std::is_nothrow_default_constructible_v<T1>;
}

template <typename T1> bool f48() { return std::is_nothrow_destructible_v<T1>; }

template <typename T1, typename T2, typename... T3> bool f49() {
  return std::is_nothrow_invocable_r_v<T1, T2, T3...>;
}

template <typename T1, typename... T2> bool f50() {
  return std::is_nothrow_invocable_v<T1, T2...>;
}

template <typename T1> bool f51() {
  return std::is_nothrow_move_assignable_v<T1>;
}

template <typename T1> bool f52() {
  return std::is_nothrow_move_constructible_v<T1>;
}

template <typename T1> bool f53() { return std::is_nothrow_swappable_v<T1>; }

template <typename T1, typename T2> bool f54() {
  return std::is_nothrow_swappable_with_v<T1, T2>;
}

template <typename T1> bool f55() { return std::is_null_pointer_v<T1>; }

template <typename T1> bool f56() { return std::is_object_v<T1>; }

template <typename T1> bool f57() { return std::is_pod_v<T1>; }

template <typename T1, typename T2> bool f58() {
  return std::is_pointer_interconvertible_base_of_v<T1, T2>;
}

template <typename T1> bool f59() { return std::is_pointer_v<T1>; }

template <typename T1> bool f60() { return std::is_polymorphic_v<T1>; }

template <typename T1> bool f61() { return std::is_reference_v<T1>; }

template <typename T1> bool f62() { return std::is_rvalue_reference_v<T1>; }

template <typename T1, typename T2> bool f63() {
  return std::is_same_v<T1, T2>;
}

template <typename T1> bool f64() { return std::is_scalar_v<T1>; }

template <typename T1> bool f65() { return std::is_scoped_enum_v<T1>; }

template <typename T1> bool f66() { return std::is_signed_v<T1>; }

template <typename T1> bool f67() { return std::is_standard_layout_v<T1>; }

template <typename T1> bool f68() { return std::is_swappable_v<T1>; }

template <typename T1, typename T2> bool f69() {
  return std::is_swappable_with_v<T1, T2>;
}

template <typename T1> bool f70() { return std::is_trivial_v<T1>; }

template <typename T1, typename T2> bool f71() {
  return std::is_trivially_assignable_v<T1, T2>;
}

template <typename T1, typename... T2> bool f72() {
  return std::is_trivially_constructible_v<T1, T2...>;
}

template <typename T1> bool f73() {
  return std::is_trivially_copy_assignable_v<T1>;
}

template <typename T1> bool f74() {
  return std::is_trivially_copy_constructible_v<T1>;
}

template <typename T1> bool f75() { return std::is_trivially_copyable_v<T1>; }

template <typename T1> bool f76() {
  return std::is_trivially_default_constructible_v<T1>;
}

template <typename T1> bool f77() {
  return std::is_trivially_destructible_v<T1>;
}

template <typename T1> bool f78() {
  return std::is_trivially_move_assignable_v<T1>;
}

template <typename T1> bool f79() {
  return std::is_trivially_move_constructible_v<T1>;
}

template <typename T1> bool f80() { return std::is_unbounded_array_v<T1>; }

template <typename T1> bool f81() { return std::is_union_v<T1>; }

template <typename T1> bool f82() { return std::is_unsigned_v<T1>; }

template <typename T1> bool f83() { return std::is_void_v<T1>; }

template <typename T1> bool f84() { return std::is_volatile_v<T1>; }

template <typename T1> using t17 = std::make_signed_t<T1>;

template <typename T1> using t18 = std::make_unsigned_t<T1>;

template <typename T1> bool f85() { return std::negation_v<T1>; }

template <typename T1> std::size_t f86() { return std::rank_v<T1>; }

template <typename T1, typename T2> bool f87() {
  return std::reference_constructs_from_temporary_v<T1, T2>;
}

template <typename T1, typename T2> bool f88() {
  return std::reference_converts_from_temporary_v<T1, T2>;
}

template <typename T1> using t19 = std::remove_all_extents_t<T1>;

template <typename T1> using t20 = std::remove_const_t<T1>;

template <typename T1> using t21 = std::remove_cv_t<T1>;

template <typename T1> using t22 = std::remove_cvref_t<T1>;

template <typename T1> using t23 = std::remove_extent_t<T1>;

template <typename T1> using t24 = std::remove_pointer_t<T1>;

template <typename T1> using t25 = std::remove_reference_t<T1>;

template <typename T1> using t26 = std::remove_volatile_t<T1>;

template <typename T1> using t27 = std::result_of_t<T1>;

using t28 = std::true_type;

template <typename T1> using t29 = std::type_identity_t<T1>;

template <typename T1> using t30 = std::underlying_type_t<T1>;

template <typename T1> using t31 = std::unwrap_ref_decay_t<T1>;

template <typename T1> using t32 = std::unwrap_reference_t<T1>;

template <typename... T1> using t33 = std::void_t<T1...>;
