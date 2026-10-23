#include <filesystem>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::filesystem::file_status;

std::filesystem::file_status f1() { return std::filesystem::file_status(); }

std::filesystem::file_status f2(std::filesystem::file_type a0,
                                std::filesystem::perms a1) {
  return std::filesystem::file_status(a0, a1);
}

std::filesystem::file_status f3(const std::filesystem::file_status &a0) {
  return std::filesystem::file_status(a0);
}

std::filesystem::file_status f4(std::filesystem::file_status &&a0) {
  return std::filesystem::file_status(std::move(a0));
}

std::filesystem::file_status &f5(std::filesystem::file_status &a0,
                                 const std::filesystem::file_status &a1) {
  return a0.operator=(a1);
}

std::filesystem::file_status &f6(std::filesystem::file_status &a0,
                                 std::filesystem::file_status &&a1) {
  return a0.operator=(std::move(a1));
}

std::filesystem::file_type f7(const std::filesystem::file_status &a0) {
  return a0.type();
}

std::filesystem::perms f8(const std::filesystem::file_status &a0) {
  return a0.permissions();
}

void f9(std::filesystem::file_status &a0, std::filesystem::file_type a1) {
  return a0.type(a1);
}

void f10(std::filesystem::file_status &a0, std::filesystem::perms a1) {
  return a0.permissions(a1);
}
