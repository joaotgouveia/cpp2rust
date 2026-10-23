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

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
using t53 = std::hash<
    std::__enable_hash_helper<std::optional<T1>, std::remove_const_t<T1>>>;

template <typename T1>
std::size_t
f254(const std::hash<std::__enable_hash_helper<std::optional<T1>,
                                               std::remove_const_t<T1>>> &a0,
     const std::optional<T1> &a1) {
  return a0.operator()(a1);
}

template <typename... T1>
using t54 = std::hash<
    std::__enable_hash_helper<std::variant<T1...>, std::remove_const_t<T1>...>>;

template <typename... T1>
std::size_t
f255(const std::hash<std::__enable_hash_helper<std::variant<T1...>,
                                               std::remove_const_t<T1>...>> &a0,
     const std::variant<T1...> &a1) {
  return a0.operator()(a1);
}
