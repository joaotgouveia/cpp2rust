#include <algorithm>
#include <cstddef>
#include <filesystem>
#include <functional>
#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <system_error>
#include <typeindex>
#include <utility>
#include <variant>
#include <vector>

template <typename T1>
using t62 = std::hash<
    std::__enable_hash_helper<std::optional<T1>, std::remove_const_t<T1>>>;

template <typename T1>
std::size_t
f255(const std::hash<std::__enable_hash_helper<std::optional<T1>,
                                               std::remove_const_t<T1>>> &a0,
     const std::optional<T1> &a1) {
  return a0.operator()(a1);
}

template <typename... T1>
using t63 = std::hash<
    std::__enable_hash_helper<std::variant<T1...>, std::remove_const_t<T1>...>>;

template <typename... T1>
std::size_t
f256(const std::hash<std::__enable_hash_helper<std::variant<T1...>,
                                               std::remove_const_t<T1>...>> &a0,
     const std::variant<T1...> &a1) {
  return a0.operator()(a1);
}
