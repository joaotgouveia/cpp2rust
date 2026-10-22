#include "../common/init.h"
#include "../common/traits.h"
#include <random>
#include <string.h>
#include <utility>

std::random_device f6(const std::string &a0) { return std::random_device(a0); }
