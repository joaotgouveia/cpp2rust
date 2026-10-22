#include "../common/init.h"
#include "../common/traits.h"
#include <filesystem>
#include <utility>

using t1 = std::filesystem::copy_options;

std::filesystem::copy_options f1() {
  return std::filesystem::copy_options::none;
}

std::filesystem::copy_options f2() {
  return std::filesystem::copy_options::skip_existing;
}

std::filesystem::copy_options f3() {
  return std::filesystem::copy_options::overwrite_existing;
}

std::filesystem::copy_options f4() {
  return std::filesystem::copy_options::update_existing;
}

std::filesystem::copy_options f5() {
  return std::filesystem::copy_options::recursive;
}

std::filesystem::copy_options f6() {
  return std::filesystem::copy_options::copy_symlinks;
}

std::filesystem::copy_options f7() {
  return std::filesystem::copy_options::skip_symlinks;
}

std::filesystem::copy_options f8() {
  return std::filesystem::copy_options::directories_only;
}

std::filesystem::copy_options f9() {
  return std::filesystem::copy_options::create_symlinks;
}

std::filesystem::copy_options f10() {
  return std::filesystem::copy_options::create_hard_links;
}

std::filesystem::copy_options f11(std::filesystem::copy_options a0,
                                  std::filesystem::copy_options a1) {
  return a0 & a1;
}

std::filesystem::copy_options &f12(std::filesystem::copy_options &a0,
                                   std::filesystem::copy_options a1) {
  return std::filesystem::operator&=(a0, a1);
}

std::filesystem::copy_options f13(std::filesystem::copy_options a0,
                                  std::filesystem::copy_options a1) {
  return a0 ^ a1;
}

std::filesystem::copy_options &f14(std::filesystem::copy_options &a0,
                                   std::filesystem::copy_options a1) {
  return std::filesystem::operator^=(a0, a1);
}

std::filesystem::copy_options f15(std::filesystem::copy_options a0,
                                  std::filesystem::copy_options a1) {
  return a0 | a1;
}

std::filesystem::copy_options &f16(std::filesystem::copy_options &a0,
                                   std::filesystem::copy_options a1) {
  return std::filesystem::operator|=(a0, a1);
}

std::filesystem::copy_options f17(std::filesystem::copy_options a0) {
  return ~a0;
}
