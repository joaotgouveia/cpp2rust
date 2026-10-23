#include <algorithm>
#include <istream>
#include <ostream>
#include <random>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

std::random_device f107(const std::string &a0) {
  return std::random_device(a0);
}
