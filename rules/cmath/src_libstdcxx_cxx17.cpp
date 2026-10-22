#include "../common/init.h"
#include "../common/traits.h"
#include <cmath>
#include <utility>

long double f220(long double a0, long double a1, long double a2) {
  return std::hypot(a0, a1, a2);
}

double f221(double a0, double a1, double a2) { return std::hypot(a0, a1, a2); }

float f222(float a0, float a1, float a2) { return std::hypot(a0, a1, a2); }

template <typename T1, typename T2, typename T3>
auto f223(T1 a0, T2 a1, T3 a2) {
  return std::hypot<T1, T2, T3>(a0, a1, a2);
}
