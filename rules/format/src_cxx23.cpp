#include <algorithm>
#include <format>
#include <ios>
#include <iosfwd>
#include <optional>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::range_format;

std::range_format f1() { return std::range_format::disabled; }

std::range_format f2() { return std::range_format::map; }

std::range_format f3() { return std::range_format::set; }

std::range_format f4() { return std::range_format::sequence; }

std::range_format f5() { return std::range_format::string; }

std::range_format f6() { return std::range_format::debug_string; }
