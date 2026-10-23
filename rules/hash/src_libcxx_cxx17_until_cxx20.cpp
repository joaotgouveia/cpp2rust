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
using t53 = typename std::hash<std::__enable_hash_helper<
    std::optional<T1>, std::remove_const_t<T1>>>::argument_type;

template <typename T1>
using t54 = typename std::hash<std::__enable_hash_helper<
    std::optional<T1>, std::remove_const_t<T1>>>::result_type;

template <typename... T1>
using t55 = typename std::hash<std::__enable_hash_helper<
    std::variant<T1...>, std::remove_const_t<T1>...>>::argument_type;

template <typename... T1>
using t56 = typename std::hash<std::__enable_hash_helper<
    std::variant<T1...>, std::remove_const_t<T1>...>>::result_type;
