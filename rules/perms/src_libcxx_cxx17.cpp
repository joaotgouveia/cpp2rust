#include <filesystem>
#include <utility>

std::filesystem::perms f1(std::filesystem::perms a0,
                          std::filesystem::perms a1) {
  return std::filesystem::operator&(a0, a1);
}

std::filesystem::perms &f2(std::filesystem::perms &a0,
                           std::filesystem::perms a1) {
  return std::filesystem::operator&=(a0, a1);
}

std::filesystem::perms f3(std::filesystem::perms a0,
                          std::filesystem::perms a1) {
  return std::filesystem::operator^(a0, a1);
}

std::filesystem::perms &f4(std::filesystem::perms &a0,
                           std::filesystem::perms a1) {
  return std::filesystem::operator^=(a0, a1);
}

std::filesystem::perms f5(std::filesystem::perms a0,
                          std::filesystem::perms a1) {
  return std::filesystem::operator|(a0, a1);
}

std::filesystem::perms &f6(std::filesystem::perms &a0,
                           std::filesystem::perms a1) {
  return std::filesystem::operator|=(a0, a1);
}

std::filesystem::perms f7(std::filesystem::perms a0) {
  return std::filesystem::operator~(a0);
}

using t1 = std::filesystem::perms;

std::filesystem::perms f8() { return std::filesystem::perms::none; }

std::filesystem::perms f9() { return std::filesystem::perms::owner_read; }

std::filesystem::perms f10() { return std::filesystem::perms::owner_write; }

std::filesystem::perms f11() { return std::filesystem::perms::owner_exec; }

std::filesystem::perms f12() { return std::filesystem::perms::owner_all; }

std::filesystem::perms f13() { return std::filesystem::perms::group_read; }

std::filesystem::perms f14() { return std::filesystem::perms::group_write; }

std::filesystem::perms f15() { return std::filesystem::perms::group_exec; }

std::filesystem::perms f16() { return std::filesystem::perms::group_all; }

std::filesystem::perms f17() { return std::filesystem::perms::others_read; }

std::filesystem::perms f18() { return std::filesystem::perms::others_write; }

std::filesystem::perms f19() { return std::filesystem::perms::others_exec; }

std::filesystem::perms f20() { return std::filesystem::perms::others_all; }

std::filesystem::perms f21() { return std::filesystem::perms::all; }

std::filesystem::perms f22() { return std::filesystem::perms::set_uid; }

std::filesystem::perms f23() { return std::filesystem::perms::set_gid; }

std::filesystem::perms f24() { return std::filesystem::perms::sticky_bit; }

std::filesystem::perms f25() { return std::filesystem::perms::mask; }

std::filesystem::perms f26() { return std::filesystem::perms::unknown; }
