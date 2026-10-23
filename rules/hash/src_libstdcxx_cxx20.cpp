#include <algorithm>
#include <atomic>
#include <barrier>
#include <bitset>
#include <chrono>
#include <coroutine>
#include <filesystem>
#include <format>
#include <string>
#include <string_view>
#include <system_error>
#include <typeindex>
#include <utility>
#include <variant>
#include <vector>

using t47 = std::hash<std::u8string_view>;

std::size_t f190(const std::hash<std::u8string_view> &a0,
                 const std::u8string_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::u8string_view> f191() { return std::hash<std::u8string_view>(); }

std::hash<std::u8string_view> f192(const std::hash<std::u8string_view> &a0) {
  return std::hash<std::u8string_view>(a0);
}

std::hash<std::u8string_view> f193(std::hash<std::u8string_view> &&a0) {
  return std::hash<std::u8string_view>(std::move(a0));
}

std::hash<std::u8string_view> &f194(std::hash<std::u8string_view> &a0,
                                    const std::hash<std::u8string_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::u8string_view> &f195(std::hash<std::u8string_view> &a0,
                                    std::hash<std::u8string_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t48 = std::hash<char8_t>;

std::size_t f196(const std::hash<char8_t> &a0, char8_t a1) {
  return a0.operator()(a1);
}

std::hash<char8_t> f197() { return std::hash<char8_t>(); }

std::hash<char8_t> f198(const std::hash<char8_t> &a0) {
  return std::hash<char8_t>(a0);
}

std::hash<char8_t> f199(std::hash<char8_t> &&a0) {
  return std::hash<char8_t>(std::move(a0));
}

std::hash<char8_t> &f200(std::hash<char8_t> &a0, const std::hash<char8_t> &a1) {
  return a0.operator=(a1);
}

std::hash<char8_t> &f201(std::hash<char8_t> &a0, std::hash<char8_t> &&a1) {
  return a0.operator=(std::move(a1));
}
