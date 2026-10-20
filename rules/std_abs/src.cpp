#include "../common/init.h"
#include "../common/traits.h"
#include <cstdlib>
#include <utility>

long double f1(long double a0) { return std::abs(a0); }

long long f2(long long a0) { return std::abs(a0); }

long f3(long a0) { return std::abs(a0); }

double f4(double a0) { return std::abs(a0); }

float f5(float a0) { return std::abs(a0); }

int f6(int a0) { return abs(a0); }
