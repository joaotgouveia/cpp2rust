#include "../common/init.h"
#include "../common/traits.h"
#include <iomanip>
#include <ios>
#include <string>
#include <string_view>
#include <utility>

auto f36(std::ios_base::fmtflags a0) { return std::resetiosflags(a0); }

auto f37(int a0) { return std::setbase(a0); }

auto f38(std::ios_base::fmtflags a0) { return std::setiosflags(a0); }

auto f39(int a0) { return std::setprecision(a0); }

auto f40(int a0) { return std::setw(a0); }
