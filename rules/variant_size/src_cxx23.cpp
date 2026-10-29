#include <algorithm>
#include <utility>
#include <variant>

template <typename T1> using t1 = std::variant_size<const T1>;

template <typename T1> using t2 = std::variant_size<volatile T1>;

template <typename T1> using t3 = std::variant_size<const volatile T1>;

template <typename... T1> using t4 = std::variant_size<std::variant<T1...>>;

template <typename... T1>
std::size_t f1(const std::variant_size<std::variant<T1...>> &a0) {
  return static_cast<std::size_t>(a0);
}

template <typename... T1>
std::size_t f2(const std::variant_size<std::variant<T1...>> &a0) {
  return a0.operator()();
}
