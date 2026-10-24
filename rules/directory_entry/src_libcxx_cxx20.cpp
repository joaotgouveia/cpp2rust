#include <compare>
#include <filesystem>
#include <ostream>
#include <system_error>
#include <utility>

std::strong_ordering f49(const std::filesystem::directory_entry &a0,
                         const std::filesystem::directory_entry &a1) {
  return a0.operator<=>(a1);
}
