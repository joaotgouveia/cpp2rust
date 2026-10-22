#include "../common/init.h"
#include "../common/traits.h"
#include <filesystem>
#include <utility>

bool f6(const std::filesystem::space_info &a0,
        const std::filesystem::space_info &a1) {
  return a0 == a1;
}
