#include <filesystem>
#include <ostream>
#include <system_error>
#include <utility>

bool f102(const std::filesystem::directory_entry &a0,
          const std::filesystem::directory_entry &a1) {
  return a0.operator!=(a1);
}

bool f103(const std::filesystem::directory_entry &a0,
          const std::filesystem::directory_entry &a1) {
  return a0.operator<(a1);
}

bool f104(const std::filesystem::directory_entry &a0,
          const std::filesystem::directory_entry &a1) {
  return a0.operator<=(a1);
}

bool f105(const std::filesystem::directory_entry &a0,
          const std::filesystem::directory_entry &a1) {
  return a0.operator>(a1);
}

bool f106(const std::filesystem::directory_entry &a0,
          const std::filesystem::directory_entry &a1) {
  return a0.operator>=(a1);
}

bool f107(const std::filesystem::directory_iterator &a0,
          const std::filesystem::directory_iterator &a1) {
  return operator!=(a0, a1);
}

bool f108(const std::filesystem::recursive_directory_iterator &a0,
          const std::filesystem::recursive_directory_iterator &a1) {
  return operator!=(a0, a1);
}
