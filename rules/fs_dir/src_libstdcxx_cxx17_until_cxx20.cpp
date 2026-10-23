#include <filesystem>
#include <system_error>
#include <utility>

bool f93(const std::filesystem::directory_entry &a0,
         const std::filesystem::directory_entry &a1) {
  return a0.operator!=(a1);
}

bool f94(const std::filesystem::directory_entry &a0,
         const std::filesystem::directory_entry &a1) {
  return a0.operator<(a1);
}

bool f95(const std::filesystem::directory_entry &a0,
         const std::filesystem::directory_entry &a1) {
  return a0.operator<=(a1);
}

bool f96(const std::filesystem::directory_entry &a0,
         const std::filesystem::directory_entry &a1) {
  return a0.operator>(a1);
}

bool f97(const std::filesystem::directory_entry &a0,
         const std::filesystem::directory_entry &a1) {
  return a0.operator>=(a1);
}

bool f98(const std::filesystem::directory_iterator &a0,
         const std::filesystem::directory_iterator &a1) {
  return operator!=(a0, a1);
}

bool f99(const std::filesystem::recursive_directory_iterator &a0,
         const std::filesystem::recursive_directory_iterator &a1) {
  return operator!=(a0, a1);
}
