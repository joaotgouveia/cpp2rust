#include <filesystem>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

bool f11(const std::filesystem::file_status &a0,
         const std::filesystem::file_status &a1) {
  return a0 == a1;
}
