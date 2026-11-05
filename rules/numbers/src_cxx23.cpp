#include <numbers>
#include <utility>

double f1() { return std::numbers::e; }

template <typename T1> T1 f2() { return std::numbers::e_v<T1>; }

double f3() { return std::numbers::egamma; }

template <typename T1> T1 f4() { return std::numbers::egamma_v<T1>; }

double f5() { return std::numbers::inv_pi; }

template <typename T1> T1 f6() { return std::numbers::inv_pi_v<T1>; }

double f7() { return std::numbers::inv_sqrt3; }

template <typename T1> T1 f8() { return std::numbers::inv_sqrt3_v<T1>; }

double f9() { return std::numbers::inv_sqrtpi; }

template <typename T1> T1 f10() { return std::numbers::inv_sqrtpi_v<T1>; }

double f11() { return std::numbers::ln10; }

template <typename T1> T1 f12() { return std::numbers::ln10_v<T1>; }

double f13() { return std::numbers::ln2; }

template <typename T1> T1 f14() { return std::numbers::ln2_v<T1>; }

double f15() { return std::numbers::log10e; }

template <typename T1> T1 f16() { return std::numbers::log10e_v<T1>; }

double f17() { return std::numbers::log2e; }

template <typename T1> T1 f18() { return std::numbers::log2e_v<T1>; }

double f19() { return std::numbers::phi; }

template <typename T1> T1 f20() { return std::numbers::phi_v<T1>; }

double f21() { return std::numbers::pi; }

template <typename T1> T1 f22() { return std::numbers::pi_v<T1>; }

double f23() { return std::numbers::sqrt2; }

template <typename T1> T1 f24() { return std::numbers::sqrt2_v<T1>; }

double f25() { return std::numbers::sqrt3; }

template <typename T1> T1 f26() { return std::numbers::sqrt3_v<T1>; }
