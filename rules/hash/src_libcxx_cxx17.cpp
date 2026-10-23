#include <algorithm>
#include <cstddef>
#include <filesystem>
#include <functional>
#include <memory>
#include <optional>
#include <system_error>
#include <typeindex>
#include <utility>
#include <variant>
#include <vector>

template <typename T1>
using t51 = std::hash<
    std::__enable_hash_helper<std::optional<T1>, std::remove_const_t<T1>>>;

template <typename T1>
std::size_t
f219(const std::hash<std::__enable_hash_helper<std::optional<T1>,
                                               std::remove_const_t<T1>>> &a0,
     const std::optional<T1> &a1) {
  return a0.operator()(a1);
}

template <typename... T1>
using t52 = std::hash<
    std::__enable_hash_helper<std::variant<T1...>, std::remove_const_t<T1>...>>;

template <typename... T1>
std::size_t
f220(const std::hash<std::__enable_hash_helper<std::variant<T1...>,
                                               std::remove_const_t<T1>...>> &a0,
     const std::variant<T1...> &a1) {
  return a0.operator()(a1);
}
