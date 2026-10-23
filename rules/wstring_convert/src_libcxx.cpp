#include <cstddef>
#include <locale>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
std::wstring_convert<T1, T2, T3, T4> f15() {
  return std::wstring_convert<T1, T2, T3, T4>();
}

template <typename T1, typename T2, typename T3, typename T4>
std::wstring_convert<T1, T2, T3, T4> f16(T1 *a0) {
  return std::wstring_convert<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::wstring_convert<T1, T2, T3, T4>
f17(T1 *a0, typename std::wstring_convert<T1, T2, T3, T4>::state_type a1) {
  return std::wstring_convert<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::wstring_convert<T1, T2, T3, T4>
f18(const typename std::wstring_convert<T1, T2, T3, T4>::byte_string &a0,
    const typename std::wstring_convert<T1, T2, T3, T4>::wide_string &a1) {
  return std::wstring_convert<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::wstring_convert<T1, T2, T3, T4>
f19(std::wstring_convert<T1, T2, T3, T4> &&a0) {
  return std::wstring_convert<T1, T2, T3, T4>(std::move(a0));
}
