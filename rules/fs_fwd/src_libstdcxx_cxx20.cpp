#include "../common/init.h"
#include "../common/traits.h"
#include <filesystem>
#include <system_error>
#include <utility>

bool f101(const std::filesystem::space_info &a0,
          const std::filesystem::space_info &a1) {
  return a0 == a1;
}
