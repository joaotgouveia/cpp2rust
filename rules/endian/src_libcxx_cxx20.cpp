#include <bit>
#include <utility>

using t1 = std::endian;

std::endian f1() { return std::endian::little; }

std::endian f2() { return std::endian::big; }

std::endian f3() { return std::endian::native; }
