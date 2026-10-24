#include <filesystem>
#include <ostream>
#include <system_error>
#include <utility>

bool f45(const std::filesystem::directory_entry &a0,
         const std::filesystem::directory_entry &a1) {
  return a0.operator!=(a1);
}

bool f46(const std::filesystem::directory_entry &a0,
         const std::filesystem::directory_entry &a1) {
  return a0.operator<(a1);
}

bool f47(const std::filesystem::directory_entry &a0,
         const std::filesystem::directory_entry &a1) {
  return a0.operator<=(a1);
}

bool f48(const std::filesystem::directory_entry &a0,
         const std::filesystem::directory_entry &a1) {
  return a0.operator>(a1);
}

bool f49(const std::filesystem::directory_entry &a0,
         const std::filesystem::directory_entry &a1) {
  return a0.operator>=(a1);
}
