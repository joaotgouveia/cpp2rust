#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <istream>
#include <ostream>
#include <random>
#include <string>
#include <utility>

std::random_device f107(const std::string &a0) {
  return std::random_device(a0);
}
