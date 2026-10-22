#include "../common/init.h"
#include "../common/traits.h"
#include <codecvt>
#include <cstddef>
#include <ios>
#include <utility>

using t41 = std::codecvt_mode;

std::codecvt_mode f76() { return std::consume_header; }

std::codecvt_mode f77() { return std::generate_header; }

std::codecvt_mode f78() { return std::little_endian; }
