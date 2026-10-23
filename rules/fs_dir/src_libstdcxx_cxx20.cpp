#include <algorithm>
#include <compare>
#include <filesystem>
#include <system_error>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

std::strong_ordering f100(const std::filesystem::directory_entry &a0,
                          const std::filesystem::directory_entry &a1) {
  return a0.operator<=>(a1);
}

bool f101(const std::filesystem::directory_iterator &a0,
          std::default_sentinel_t a1) {
  return a0.operator==(std::move(a1));
}

bool f102(const std::filesystem::file_status &a0,
          const std::filesystem::file_status &a1) {
  return a0 == a1;
}

bool f103(const std::filesystem::recursive_directory_iterator &a0,
          std::default_sentinel_t a1) {
  return a0.operator==(std::move(a1));
}
