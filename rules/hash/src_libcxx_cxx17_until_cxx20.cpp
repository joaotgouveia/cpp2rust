#include <algorithm>
#include <bitset>
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
using t66 = typename std::hash<std::__enable_hash_helper<
    std::optional<T1>, std::remove_const_t<T1>>>::argument_type;

template <typename T1>
using t67 = typename std::hash<std::__enable_hash_helper<
    std::optional<T1>, std::remove_const_t<T1>>>::result_type;

template <typename... T1>
using t68 = typename std::hash<std::__enable_hash_helper<
    std::variant<T1...>, std::remove_const_t<T1>...>>::argument_type;

template <typename... T1>
using t69 = typename std::hash<std::__enable_hash_helper<
    std::variant<T1...>, std::remove_const_t<T1>...>>::result_type;
