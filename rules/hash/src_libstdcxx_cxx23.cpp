#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <barrier>
#include <bitset>
#include <chrono>
#include <coroutine>
#include <filesystem>
#include <format>
#include <optional>
#include <stacktrace>
#include <string_view>
#include <system_error>
#include <typeindex>
#include <utility>
#include <vector>

using t40 = std::hash<std::stacktrace_entry>;

std::size_t f197(const std::hash<std::stacktrace_entry> &a0,
                 const std::stacktrace_entry &a1) {
  return a0.operator()(a1);
}

std::hash<std::stacktrace_entry> f198() {
  return std::hash<std::stacktrace_entry>();
}

std::hash<std::stacktrace_entry>
f199(const std::hash<std::stacktrace_entry> &a0) {
  return std::hash<std::stacktrace_entry>(a0);
}

std::hash<std::stacktrace_entry> f200(std::hash<std::stacktrace_entry> &&a0) {
  return std::hash<std::stacktrace_entry>(std::move(a0));
}

std::hash<std::stacktrace_entry> &
f201(std::hash<std::stacktrace_entry> &a0,
     const std::hash<std::stacktrace_entry> &a1) {
  return a0.operator=(a1);
}

std::hash<std::stacktrace_entry> &f202(std::hash<std::stacktrace_entry> &a0,
                                       std::hash<std::stacktrace_entry> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> using t41 = std::hash<std::basic_stacktrace<T1>>;

template <typename T1>
std::size_t f203(const std::hash<std::basic_stacktrace<T1>> &a0,
                 const std::basic_stacktrace<T1> &a1) {
  return a0.operator()(a1);
}
