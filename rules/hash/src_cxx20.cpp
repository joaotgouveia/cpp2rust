#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <atomic>
#include <barrier>
#include <bitset>
#include <chrono>
#include <coroutine>
#include <cstddef>
#include <filesystem>
#include <format>
#include <functional>
#include <memory>
#include <optional>
#include <string_view>
#include <system_error>
#include <typeindex>
#include <utility>
#include <variant>
#include <vector>

template <typename T1> using t10 = std::hash<std::coroutine_handle<T1>>;

template <typename T1>
std::size_t f27(const std::hash<std::coroutine_handle<T1>> &a0,
                const std::coroutine_handle<T1> &a1) {
  return a0.operator()(a1);
}
