#include <algorithm>
#include <atomic>
#include <barrier>
#include <bitset>
#include <chrono>
#include <coroutine>
#include <filesystem>
#include <format>
#include <string_view>
#include <system_error>
#include <typeindex>
#include <utility>
#include <vector>

#include "../common/init.h"
#include "../common/traits.h"

using t38 = std::hash<std::u8string_view>;

std::size_t f185(const std::hash<std::u8string_view> &a0,
                 const std::u8string_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::u8string_view> f186() { return std::hash<std::u8string_view>(); }

std::hash<std::u8string_view> f187(const std::hash<std::u8string_view> &a0) {
  return std::hash<std::u8string_view>(a0);
}

std::hash<std::u8string_view> f188(std::hash<std::u8string_view> &&a0) {
  return std::hash<std::u8string_view>(std::move(a0));
}

std::hash<std::u8string_view> &f189(std::hash<std::u8string_view> &a0,
                                    const std::hash<std::u8string_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::u8string_view> &f190(std::hash<std::u8string_view> &a0,
                                    std::hash<std::u8string_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t39 = std::hash<char8_t>;

std::size_t f191(const std::hash<char8_t> &a0, char8_t a1) {
  return a0.operator()(a1);
}

std::hash<char8_t> f192() { return std::hash<char8_t>(); }

std::hash<char8_t> f193(const std::hash<char8_t> &a0) {
  return std::hash<char8_t>(a0);
}

std::hash<char8_t> f194(std::hash<char8_t> &&a0) {
  return std::hash<char8_t>(std::move(a0));
}

std::hash<char8_t> &f195(std::hash<char8_t> &a0, const std::hash<char8_t> &a1) {
  return a0.operator=(a1);
}

std::hash<char8_t> &f196(std::hash<char8_t> &a0, std::hash<char8_t> &&a1) {
  return a0.operator=(std::move(a1));
}
