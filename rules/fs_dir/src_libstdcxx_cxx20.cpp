#include <algorithm>
#include <barrier>
#include <compare>
#include <filesystem>
#include <system_error>
#include <utility>

std::strong_ordering f109(const std::filesystem::directory_entry &a0,
                          const std::filesystem::directory_entry &a1) {
  return a0.operator<=>(a1);
}

bool f110(const std::filesystem::directory_iterator &a0,
          std::default_sentinel_t a1) {
  return a0.operator==(std::move(a1));
}

bool f111(const std::filesystem::file_status &a0,
          const std::filesystem::file_status &a1) {
  return operator==(a0, a1);
}

bool f112(const std::filesystem::recursive_directory_iterator &a0,
          std::default_sentinel_t a1) {
  return a0.operator==(std::move(a1));
}
