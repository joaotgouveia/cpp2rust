#include <initializer_list>
#include <ios>
#include <iosfwd>
#include <locale>
#include <string>
#include <utility>

template <typename T1>
bool f78(const typename std::messages<T1>::string_type &a0, T1 a1) {
  return a0.starts_with(a1);
}

template <typename T1>
bool f79(const typename std::messages<T1>::string_type &a0, T1 const *a1) {
  return a0.starts_with(a1);
}

template <typename T1>
bool f80(const typename std::messages<T1>::string_type &a0, T1 a1) {
  return a0.ends_with(a1);
}

template <typename T1>
bool f81(const typename std::messages<T1>::string_type &a0, T1 const *a1) {
  return a0.ends_with(a1);
}
