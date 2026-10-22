#include "../common/init.h"
#include "../common/traits.h"
#include <cstddef>
#include <iomanip>
#include <ios>
#include <utility>

template <typename T1, typename T2>
using t4 = typename std::time_get<T1, T2>::dateorder;

template <typename T1, typename T2>
using t5 = typename std::time_get<T1, T2>::string_type;

template <typename T1, typename T2> long f11(const std::time_get<T1, T2> &a0) {
  return a0.use_count();
}

template <typename T1, typename T2>
using t6 = typename std::time_get<T1, T2>::string_type::traits_type;

template <typename T1, typename T2>
using t7 = typename std::time_get<T1, T2>::string_type::value_type;

template <typename T1, typename T2>
using t8 = typename std::time_get<T1, T2>::string_type::allocator_type;

template <typename T1, typename T2>
using t9 = typename std::time_get<T1, T2>::string_type::size_type;

template <typename T1, typename T2>
using t10 = typename std::time_get<T1, T2>::string_type::difference_type;

template <typename T1, typename T2>
using t11 = typename std::time_get<T1, T2>::string_type::reference;

template <typename T1, typename T2>
using t12 = typename std::time_get<T1, T2>::string_type::const_reference;

template <typename T1, typename T2>
using t13 = typename std::time_get<T1, T2>::string_type::pointer;

template <typename T1, typename T2>
using t14 = typename std::time_get<T1, T2>::string_type::const_pointer;
