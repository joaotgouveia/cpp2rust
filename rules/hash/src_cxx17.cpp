#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <bitset>
#include <condition_variable>
#include <cstddef>
#include <filesystem>
#include <functional>
#include <future>
#include <memory>
#include <optional>
#include <string_view>
#include <system_error>
#include <typeindex>
#include <utility>
#include <variant>
#include <vector>

using t8 = std::hash<std::filesystem::path>;

std::size_t f15(const std::hash<std::filesystem::path> &a0,
                const std::filesystem::path &a1) {
  return a0.operator()(a1);
}

std::hash<std::filesystem::path> f16() {
  return std::hash<std::filesystem::path>();
}

std::hash<std::filesystem::path>
f17(const std::hash<std::filesystem::path> &a0) {
  return std::hash<std::filesystem::path>(a0);
}

std::hash<std::filesystem::path> f18(std::hash<std::filesystem::path> &&a0) {
  return std::hash<std::filesystem::path>(std::move(a0));
}

std::hash<std::filesystem::path> &
f19(std::hash<std::filesystem::path> &a0,
    const std::hash<std::filesystem::path> &a1) {
  return a0.operator=(a1);
}

std::hash<std::filesystem::path> &f20(std::hash<std::filesystem::path> &a0,
                                      std::hash<std::filesystem::path> &&a1) {
  return a0.operator=(std::move(a1));
}

using t9 = std::hash<std::monostate>;

std::size_t f21(const std::hash<std::monostate> &a0, const std::monostate &a1) {
  return a0.operator()(a1);
}

std::hash<std::monostate> f22() { return std::hash<std::monostate>(); }

std::hash<std::monostate> f23(const std::hash<std::monostate> &a0) {
  return std::hash<std::monostate>(a0);
}

std::hash<std::monostate> f24(std::hash<std::monostate> &&a0) {
  return std::hash<std::monostate>(std::move(a0));
}

std::hash<std::monostate> &f25(std::hash<std::monostate> &a0,
                               const std::hash<std::monostate> &a1) {
  return a0.operator=(a1);
}

std::hash<std::monostate> &f26(std::hash<std::monostate> &a0,
                               std::hash<std::monostate> &&a1) {
  return a0.operator=(std::move(a1));
}
