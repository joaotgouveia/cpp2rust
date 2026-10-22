#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <ios>
#include <iosfwd>
#include <utility>

template <typename T1>
bool f75(const typename std::collate_byname<T1>::string_type &a0,
         std::basic_string_view<T1, std::char_traits<T1>> a1) {
  return a0.starts_with(a1);
}

template <typename T1>
bool f76(const typename std::collate_byname<T1>::string_type &a0, T1 a1) {
  return a0.starts_with(a1);
}

template <typename T1>
bool f77(const typename std::collate_byname<T1>::string_type &a0,
         T1 const *a1) {
  return a0.starts_with(a1);
}

template <typename T1>
bool f78(const typename std::collate_byname<T1>::string_type &a0,
         std::basic_string_view<T1, std::char_traits<T1>> a1) {
  return a0.ends_with(a1);
}

template <typename T1>
bool f79(const typename std::collate_byname<T1>::string_type &a0, T1 a1) {
  return a0.ends_with(a1);
}

template <typename T1>
bool f80(const typename std::collate_byname<T1>::string_type &a0,
         T1 const *a1) {
  return a0.ends_with(a1);
}
