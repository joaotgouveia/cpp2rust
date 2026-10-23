#include <algorithm>
#include <barrier>
#include <bitset>
#include <chrono>
#include <coroutine>
#include <filesystem>
#include <format>
#include <optional>
#include <stacktrace>
#include <string>
#include <string_view>
#include <system_error>
#include <typeindex>
#include <utility>
#include <variant>
#include <vector>

using t49 = std::hash<std::stacktrace_entry>;

std::size_t f202(const std::hash<std::stacktrace_entry> &a0,
                 const std::stacktrace_entry &a1) {
  return a0.operator()(a1);
}

std::hash<std::stacktrace_entry> f203() {
  return std::hash<std::stacktrace_entry>();
}

std::hash<std::stacktrace_entry>
f204(const std::hash<std::stacktrace_entry> &a0) {
  return std::hash<std::stacktrace_entry>(a0);
}

std::hash<std::stacktrace_entry> f205(std::hash<std::stacktrace_entry> &&a0) {
  return std::hash<std::stacktrace_entry>(std::move(a0));
}

std::hash<std::stacktrace_entry> &
f206(std::hash<std::stacktrace_entry> &a0,
     const std::hash<std::stacktrace_entry> &a1) {
  return a0.operator=(a1);
}

std::hash<std::stacktrace_entry> &f207(std::hash<std::stacktrace_entry> &a0,
                                       std::hash<std::stacktrace_entry> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> using t50 = std::hash<std::basic_stacktrace<T1>>;

template <typename T1>
std::size_t f208(const std::hash<std::basic_stacktrace<T1>> &a0,
                 const std::basic_stacktrace<T1> &a1) {
  return a0.operator()(a1);
}
