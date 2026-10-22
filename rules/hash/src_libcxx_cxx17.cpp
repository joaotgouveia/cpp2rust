#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <bitset>
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
using t54 = std::hash<
    std::__enable_hash_helper<std::optional<T1>, std::remove_const_t<T1>>>;

template <typename T1>
std::size_t
f256(const std::hash<std::__enable_hash_helper<std::optional<T1>,
                                               std::remove_const_t<T1>>> &a0,
     const std::optional<T1> &a1) {
  return a0.operator()(a1);
}

template <typename... T1>
using t55 = std::hash<
    std::__enable_hash_helper<std::variant<T1...>, std::remove_const_t<T1>...>>;

template <typename... T1>
std::size_t
f257(const std::hash<std::__enable_hash_helper<std::variant<T1...>,
                                               std::remove_const_t<T1>...>> &a0,
     const std::variant<T1...> &a1) {
  return a0.operator()(a1);
}
