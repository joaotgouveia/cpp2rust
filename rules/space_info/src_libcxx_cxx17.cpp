#include "../common/init.h"
#include "../common/traits.h"
#include <filesystem>
#include <utility>

using t1 = std::filesystem::space_info;

std::filesystem::space_info f1() { return std::filesystem::space_info(); }

std::filesystem::space_info f2(const std::filesystem::space_info &a0) {
  return std::filesystem::space_info(a0);
}

std::filesystem::space_info f3(std::filesystem::space_info &&a0) {
  return std::filesystem::space_info(std::move(a0));
}

std::filesystem::space_info &f4(std::filesystem::space_info &a0,
                                const std::filesystem::space_info &a1) {
  return a0.operator=(a1);
}

std::filesystem::space_info &f5(std::filesystem::space_info &a0,
                                std::filesystem::space_info &&a1) {
  return a0.operator=(std::move(a1));
}
