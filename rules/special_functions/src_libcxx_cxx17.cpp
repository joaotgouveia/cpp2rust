#include <cmath>
#include <utility>

long double f1(unsigned int a0, long double a1) { return std::hermite(a0, a1); }

double f2(unsigned int a0, double a1) { return std::hermite(a0, a1); }

float f3(unsigned int a0, float a1) { return std::hermite(a0, a1); }

template <typename T1, std::enable_if_t<std::is_integral_v<T1>, int> T2 = 0>
double f4(unsigned int a0, T1 a1) {
  return std::hermite<T1>(a0, a1);
}

float f5(unsigned int a0, float a1) { return std::hermitef(a0, a1); }

long double f6(unsigned int a0, long double a1) {
  return std::hermitel(a0, a1);
}
