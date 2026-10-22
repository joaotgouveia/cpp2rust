#include "../common/init.h"
#include "../common/traits.h"
#include <cmath>
#include <utility>

long double f1(long double a0, long double a1, long double a2) {
  return std::lerp(a0, a1, a2);
}

double f2(double a0, double a1, double a2) { return std::lerp(a0, a1, a2); }

float f3(float a0, float a1, float a2) { return std::lerp(a0, a1, a2); }
