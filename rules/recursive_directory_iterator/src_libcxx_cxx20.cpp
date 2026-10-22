#include "../common/init.h"
#include "../common/traits.h"
#include <filesystem>
#include <iterator>
#include <system_error>
#include <utility>

bool f23(const std::filesystem::recursive_directory_iterator &a0,
         std::default_sentinel_t a1) {
  return a0.operator==(std::move(a1));
}
