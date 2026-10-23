#include <algorithm>
#include <barrier>
#include <chrono>
#include <cstddef>
#include <format>
#include <queue>
#include <stack>
#include <string>
#include <string_view>
#include <thread>
#include <tuple>
#include <type_traits>
#include <utility>
#include <vector>

void f6(std::formatter<char, wchar_t> &a0) { return a0.set_debug_format(); }

template <typename T1> void f7(std::formatter<const T1 *, T1> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename... T2>
using t25 = std::formatter<std::tuple<T2...>, T1>;

template <typename T1, typename... T2>
void f8(std::formatter<std::tuple<T2...>, T1> &a0,
        std::basic_string_view<T1> a1) {
  return a0.set_separator(a1);
}

template <typename T1, typename... T2>
void f9(std::formatter<std::tuple<T2...>, T1> &a0,
        std::basic_string_view<T1> a1, std::basic_string_view<T1> a2) {
  return a0.set_brackets(a1, a2);
}

template <typename T1, typename T2, typename T3>
using t26 = std::formatter<std::queue<T2, T3>, T1>;

template <typename T1, typename T2, typename T3>
using t27 = std::formatter<std::stack<T2, T3>, T1>;

template <typename T1, typename T2, typename T3, typename T4>
using t28 = std::formatter<std::priority_queue<T2, T3, T4>, T1>;
