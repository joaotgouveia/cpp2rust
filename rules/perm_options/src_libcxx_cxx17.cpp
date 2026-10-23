#include <filesystem>
#include <utility>

std::filesystem::perm_options f1(std::filesystem::perm_options a0,
                                 std::filesystem::perm_options a1) {
  return std::filesystem::operator&(a0, a1);
}

std::filesystem::perm_options &f2(std::filesystem::perm_options &a0,
                                  std::filesystem::perm_options a1) {
  return std::filesystem::operator&=(a0, a1);
}

std::filesystem::perm_options f3(std::filesystem::perm_options a0,
                                 std::filesystem::perm_options a1) {
  return std::filesystem::operator^(a0, a1);
}

std::filesystem::perm_options &f4(std::filesystem::perm_options &a0,
                                  std::filesystem::perm_options a1) {
  return std::filesystem::operator^=(a0, a1);
}

std::filesystem::perm_options f5(std::filesystem::perm_options a0,
                                 std::filesystem::perm_options a1) {
  return std::filesystem::operator|(a0, a1);
}

std::filesystem::perm_options &f6(std::filesystem::perm_options &a0,
                                  std::filesystem::perm_options a1) {
  return std::filesystem::operator|=(a0, a1);
}

std::filesystem::perm_options f7(std::filesystem::perm_options a0) {
  return std::filesystem::operator~(a0);
}

using t1 = std::filesystem::perm_options;

std::filesystem::perm_options f8() {
  return std::filesystem::perm_options::replace;
}

std::filesystem::perm_options f9() {
  return std::filesystem::perm_options::add;
}

std::filesystem::perm_options f10() {
  return std::filesystem::perm_options::remove;
}

std::filesystem::perm_options f11() {
  return std::filesystem::perm_options::nofollow;
}
