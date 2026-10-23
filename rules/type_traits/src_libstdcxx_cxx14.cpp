#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t3 = std::add_const_t<T1>;

template <typename T1> using t4 = std::add_cv_t<T1>;

template <typename T1> using t5 = std::add_lvalue_reference_t<T1>;

template <typename T1> using t6 = std::add_pointer_t<T1>;

template <typename T1> using t7 = std::add_rvalue_reference_t<T1>;

template <typename T1> using t8 = std::add_volatile_t<T1>;

template <std::size_t T1, std::size_t T2>
using t9 = std::aligned_storage_t<T1, T2>;

template <std::size_t T1, typename... T2>
using t10 = std::aligned_union_t<T1, T2...>;

template <typename... T1> using t11 = std::common_type_t<T1...>;

template <bool T1, typename T2, typename T3>
using t12 = std::conditional_t<T1, T2, T3>;

template <typename T1> using t13 = std::decay_t<T1>;

template <bool T1, typename T2> using t14 = std::enable_if_t<T1, T2>;

template <typename T1> using t15 = std::make_signed_t<T1>;

template <typename T1> using t16 = std::make_unsigned_t<T1>;

template <typename T1> using t17 = std::remove_all_extents_t<T1>;

template <typename T1> using t18 = std::remove_const_t<T1>;

template <typename T1> using t19 = std::remove_cv_t<T1>;

template <typename T1> using t20 = std::remove_extent_t<T1>;

template <typename T1> using t21 = std::remove_pointer_t<T1>;

template <typename T1> using t22 = std::remove_reference_t<T1>;

template <typename T1> using t23 = std::remove_volatile_t<T1>;

template <typename T1> using t24 = std::result_of_t<T1>;

template <typename T1> using t25 = std::underlying_type_t<T1>;
