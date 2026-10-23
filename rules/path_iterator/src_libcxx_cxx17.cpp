#include <filesystem>
#include <utility>

bool f1(const std::filesystem::path::iterator &a0,
        const std::filesystem::path::iterator &a1) {
  return std::filesystem::operator!=(a0, a1);
}

bool f2(const std::filesystem::path::iterator &a0,
        const std::filesystem::path::iterator &a1) {
  return std::filesystem::operator==(a0, a1);
}
