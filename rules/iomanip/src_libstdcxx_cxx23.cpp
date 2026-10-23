#include <iomanip>
#include <ios>
#include <string>
#include <string_view>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

auto f25(std::ios_base::fmtflags a0) { return std::resetiosflags(a0); }

auto f26(int a0) { return std::setbase(a0); }

auto f27(std::ios_base::fmtflags a0) { return std::setiosflags(a0); }

auto f28(int a0) { return std::setprecision(a0); }

auto f29(int a0) { return std::setw(a0); }
