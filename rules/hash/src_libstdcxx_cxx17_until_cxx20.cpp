#include <algorithm>
#include <bitset>
#include <condition_variable>
#include <filesystem>
#include <future>
#include <optional>
#include <string>
#include <string_view>
#include <system_error>
#include <typeindex>
#include <utility>
#include <variant>
#include <vector>

template <typename T1, typename std::enable_if<
                           std::is_base_of<std::__optional_hash<T1>,
                                           std::hash<std::optional<T1>>>::value,
                           int>::type T2 = 0>
std::size_t f200(const std::hash<std::optional<T1>> &a0,
                 const std::optional<T1> &a1) {
  return a0.operator()(a1);
}

template <typename... T1,
          typename std::enable_if<
              std::is_base_of<std::__variant_hash<T1...>,
                              std::hash<std::variant<T1...>>>::value,
              int>::type T2 = 0>
std::size_t f201(const std::hash<std::variant<T1...>> &a0,
                 const std::variant<T1...> &a1) {
  return a0.operator()(a1);
}
