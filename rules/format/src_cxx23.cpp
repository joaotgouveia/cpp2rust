#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <format>
#include <ios>
#include <iosfwd>
#include <utility>

using t10 = std::range_format;

std::range_format f35() { return std::range_format::disabled; }

std::range_format f36() { return std::range_format::map; }

std::range_format f37() { return std::range_format::set; }

std::range_format f38() { return std::range_format::sequence; }

std::range_format f39() { return std::range_format::string; }

std::range_format f40() { return std::range_format::debug_string; }
