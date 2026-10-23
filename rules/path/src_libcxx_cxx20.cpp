#include <compare>
#include <cstddef>
#include <filesystem>
#include <string.h>
#include <string>
#include <string_view>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

std::u8string f78(const std::filesystem::path &a0) { return a0.u8string(); }

std::u8string f79(const std::filesystem::path &a0) {
  return a0.generic_u8string();
}

std::strong_ordering f80(const std::filesystem::path &a0,
                         const std::filesystem::path &a1) {
  return a0 <=> a1;
}
