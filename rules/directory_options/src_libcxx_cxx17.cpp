#include <filesystem>
#include <utility>

using t1 = std::filesystem::directory_options;

std::filesystem::directory_options f1() {
  return std::filesystem::directory_options::none;
}

std::filesystem::directory_options f2() {
  return std::filesystem::directory_options::follow_directory_symlink;
}

std::filesystem::directory_options f3() {
  return std::filesystem::directory_options::skip_permission_denied;
}

std::filesystem::directory_options f4(std::filesystem::directory_options a0,
                                      std::filesystem::directory_options a1) {
  return std::filesystem::operator&(a0, a1);
}

std::filesystem::directory_options &f5(std::filesystem::directory_options &a0,
                                       std::filesystem::directory_options a1) {
  return std::filesystem::operator&=(a0, a1);
}

std::filesystem::directory_options f6(std::filesystem::directory_options a0,
                                      std::filesystem::directory_options a1) {
  return std::filesystem::operator^(a0, a1);
}

std::filesystem::directory_options &f7(std::filesystem::directory_options &a0,
                                       std::filesystem::directory_options a1) {
  return std::filesystem::operator^=(a0, a1);
}

std::filesystem::directory_options f8(std::filesystem::directory_options a0,
                                      std::filesystem::directory_options a1) {
  return std::filesystem::operator|(a0, a1);
}

std::filesystem::directory_options &f9(std::filesystem::directory_options &a0,
                                       std::filesystem::directory_options a1) {
  return std::filesystem::operator|=(a0, a1);
}

std::filesystem::directory_options f10(std::filesystem::directory_options a0) {
  return std::filesystem::operator~(a0);
}
