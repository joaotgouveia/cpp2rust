#include <filesystem>
#include <utility>

bool f11(const std::filesystem::file_status &a0,
         const std::filesystem::file_status &a1) {
  return operator==(a0, a1);
}
