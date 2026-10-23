#include <algorithm>
#include <locale>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3, typename T4>
std::wstring_convert<T1, T2, T3, T4> f11() {
  return {};
}

template <typename T1, typename T2, typename T3, typename T4>
std::wstring_convert<T1, T2, T3, T4>
f12(T1 *a0, typename std::wstring_convert<T1, T2, T3, T4>::state_type a1) {
  return {a0, a1};
}
