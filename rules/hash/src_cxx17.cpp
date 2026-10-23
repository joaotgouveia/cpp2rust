#include <algorithm>
#include <bitset>
#include <condition_variable>
#include <cstddef>
#include <filesystem>
#include <functional>
#include <future>
#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <system_error>
#include <typeindex>
#include <utility>
#include <variant>
#include <vector>

using t11 = std::hash<std::filesystem::path>;

std::size_t f19(const std::hash<std::filesystem::path> &a0,
                const std::filesystem::path &a1) {
  return a0.operator()(a1);
}

std::hash<std::filesystem::path> f20() {
  return std::hash<std::filesystem::path>();
}

std::hash<std::filesystem::path>
f21(const std::hash<std::filesystem::path> &a0) {
  return std::hash<std::filesystem::path>(a0);
}

std::hash<std::filesystem::path> f22(std::hash<std::filesystem::path> &&a0) {
  return std::hash<std::filesystem::path>(std::move(a0));
}

std::hash<std::filesystem::path> &
f23(std::hash<std::filesystem::path> &a0,
    const std::hash<std::filesystem::path> &a1) {
  return a0.operator=(a1);
}

std::hash<std::filesystem::path> &f24(std::hash<std::filesystem::path> &a0,
                                      std::hash<std::filesystem::path> &&a1) {
  return a0.operator=(std::move(a1));
}

using t12 = std::hash<std::monostate>;

std::size_t f25(const std::hash<std::monostate> &a0, const std::monostate &a1) {
  return a0.operator()(a1);
}

std::hash<std::monostate> f26() { return std::hash<std::monostate>(); }

std::hash<std::monostate> f27(const std::hash<std::monostate> &a0) {
  return std::hash<std::monostate>(a0);
}

std::hash<std::monostate> f28(std::hash<std::monostate> &&a0) {
  return std::hash<std::monostate>(std::move(a0));
}

std::hash<std::monostate> &f29(std::hash<std::monostate> &a0,
                               const std::hash<std::monostate> &a1) {
  return a0.operator=(a1);
}

std::hash<std::monostate> &f30(std::hash<std::monostate> &a0,
                               std::hash<std::monostate> &&a1) {
  return a0.operator=(std::move(a1));
}
