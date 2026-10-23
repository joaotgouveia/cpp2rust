#include <filesystem>
#include <system_error>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

bool f43(const std::filesystem::directory_entry &a0,
         const std::filesystem::directory_entry &a1) {
  return a0.operator!=(a1);
}

bool f44(const std::filesystem::directory_entry &a0,
         const std::filesystem::directory_entry &a1) {
  return a0.operator<(a1);
}

bool f45(const std::filesystem::directory_entry &a0,
         const std::filesystem::directory_entry &a1) {
  return a0.operator<=(a1);
}

bool f46(const std::filesystem::directory_entry &a0,
         const std::filesystem::directory_entry &a1) {
  return a0.operator>(a1);
}

bool f47(const std::filesystem::directory_entry &a0,
         const std::filesystem::directory_entry &a1) {
  return a0.operator>=(a1);
}
