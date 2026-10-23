#include <filesystem>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::filesystem::file_type;

std::filesystem::file_type f1() { return std::filesystem::file_type::none; }

std::filesystem::file_type f2() {
  return std::filesystem::file_type::not_found;
}

std::filesystem::file_type f3() { return std::filesystem::file_type::regular; }

std::filesystem::file_type f4() {
  return std::filesystem::file_type::directory;
}

std::filesystem::file_type f5() { return std::filesystem::file_type::symlink; }

std::filesystem::file_type f6() { return std::filesystem::file_type::block; }

std::filesystem::file_type f7() {
  return std::filesystem::file_type::character;
}

std::filesystem::file_type f8() { return std::filesystem::file_type::fifo; }

std::filesystem::file_type f9() { return std::filesystem::file_type::socket; }

std::filesystem::file_type f10() { return std::filesystem::file_type::unknown; }
