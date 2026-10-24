#include <algorithm>
#include <utility>
#include <valarray>

bool f35(const std::slice &a0, const std::slice &a1) {
  return operator==(a0, a1);
}
