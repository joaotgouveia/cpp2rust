#include <algorithm>
#include <initializer_list>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f131(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reference
f132(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reference
f133(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reference
f134(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reference
f135(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reference
f136(std::basic_string<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reference
f137(const std::basic_string<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reference
f138(std::basic_string<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reference
f139(const std::basic_string<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f140(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, T1 *a3,
     T1 *a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f141(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, const T1 *a3,
     const T1 *a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f142(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2,
     typename std::basic_string<T1, T2, T3>::iterator a3,
     typename std::basic_string<T1, T2, T3>::iterator a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f143(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2,
     typename std::basic_string<T1, T2, T3>::const_iterator a3,
     typename std::basic_string<T1, T2, T3>::const_iterator a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = std::_RequireInputIter<T4>>
std::basic_string<T1, T2, T3> f144(T4 a0, T4 a1, const T3 &a2) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = std::_RequireInputIter<T4>>
std::basic_string<T1, T2, T3> &f145(std::basic_string<T1, T2, T3> &a0, T4 a1,
                                    T4 a2) {
  return a0.template append<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = std::_RequireInputIter<T4>>
std::basic_string<T1, T2, T3> &f146(std::basic_string<T1, T2, T3> &a0, T4 a1,
                                    T4 a2) {
  return a0.template assign<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = std::_RequireInputIter<T4>>
typename std::basic_string<T1, T2, T3>::iterator
f147(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1, T4 a2, T4 a3) {
  return a0.template insert<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = std::_RequireInputIter<T4>>
std::basic_string<T1, T2, T3> &
f148(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, T4 a3, T4 a4) {
  return a0.template replace<T4>(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
using t23 = typename std::basic_string<T1, T2, T3>::iterator::iterator_type;

template <typename T1, typename T2, typename T3>
using t24 =
    typename std::basic_string<T1, T2, T3>::const_iterator::iterator_type;

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f149(std::basic_istream<T1, T2> &&a0,
                                 std::basic_string<T1, T2, T3> &a1) {
  return std::getline<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f150(std::basic_istream<T1, T2> &&a0,
                                 std::basic_string<T1, T2, T3> &a1, T1 a2) {
  return std::getline<T1, T2, T3>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f151(std::basic_istream<T1, T2> &a0,
                                 std::basic_string<T1, T2, T3> &a1) {
  return std::getline<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f152(std::basic_istream<T1, T2> &a0,
                                 std::basic_string<T1, T2, T3> &a1, T1 a2) {
  return std::getline<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f153(std::basic_string<T1, T2, T3> &&a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return std::operator+ <T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f154(std::basic_string<T1, T2, T3> &&a0,
                                   const T1 *a1) {
  return std::operator+ <T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f155(std::basic_string<T1, T2, T3> &&a0,
                                   std::basic_string<T1, T2, T3> &&a1) {
  return std::operator+ <T1, T2, T3>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f156(std::basic_string<T1, T2, T3> &&a0, T1 a1) {
  return std::operator+ <T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f157(const std::basic_string<T1, T2, T3> &a0,
                                   std::basic_string<T1, T2, T3> &&a1) {
  return std::operator+ <T1, T2, T3>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f158(const std::basic_string<T1, T2, T3> &a0,
                                   const T1 *a1) {
  return std::operator+ <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f159(const std::basic_string<T1, T2, T3> &a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return std::operator+ <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f160(const std::basic_string<T1, T2, T3> &a0,
                                   T1 a1) {
  return std::operator+ <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f161(const T1 *a0,
                                   std::basic_string<T1, T2, T3> &&a1) {
  return std::operator+ <T1, T2, T3>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f162(const T1 *a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return std::operator+ <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f163(T1 a0, std::basic_string<T1, T2, T3> &&a1) {
  return std::operator+ <T1, T2, T3>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f164(T1 a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return std::operator+ <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f165(std::basic_ostream<T1, T2> &a0,
                                 const std::basic_string<T1, T2, T3> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f166(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return std::operator== <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f167(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return std::operator== <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f168(std::basic_istream<T1, T2> &a0,
                                 std::basic_string<T1, T2, T3> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

double f169(const std::string &a0, std::size_t *a1) {
  return std::stod(a0, a1);
}

double f170(const std::wstring &a0, std::size_t *a1) {
  return std::stod(a0, a1);
}

float f171(const std::string &a0, std::size_t *a1) { return std::stof(a0, a1); }

float f172(const std::wstring &a0, std::size_t *a1) {
  return std::stof(a0, a1);
}

int f173(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoi(a0, a1, a2);
}

int f174(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoi(a0, a1, a2);
}

long f175(const std::string &a0, std::size_t *a1, int a2) {
  return std::stol(a0, a1, a2);
}

long f176(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stol(a0, a1, a2);
}

long double f177(const std::string &a0, std::size_t *a1) {
  return std::stold(a0, a1);
}

long double f178(const std::wstring &a0, std::size_t *a1) {
  return std::stold(a0, a1);
}

long long f179(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoll(a0, a1, a2);
}

long long f180(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoll(a0, a1, a2);
}

unsigned long f181(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoul(a0, a1, a2);
}

unsigned long f182(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoul(a0, a1, a2);
}

unsigned long long f183(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoull(a0, a1, a2);
}

unsigned long long f184(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoull(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
void f185(std::basic_string<T1, T2, T3> &a0,
          std::basic_string<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}

std::string f186(long double a0) { return std::to_string(a0); }

std::string f187(int a0) { return std::to_string(a0); }

std::string f188(long long a0) { return std::to_string(a0); }

std::string f189(long a0) { return std::to_string(a0); }

std::string f190(double a0) { return std::to_string(a0); }

std::string f191(float a0) { return std::to_string(a0); }

std::string f192(unsigned int a0) { return std::to_string(a0); }

std::string f193(unsigned long long a0) { return std::to_string(a0); }

std::string f194(unsigned long a0) { return std::to_string(a0); }

std::wstring f195(long double a0) { return std::to_wstring(a0); }

std::wstring f196(int a0) { return std::to_wstring(a0); }

std::wstring f197(long long a0) { return std::to_wstring(a0); }

std::wstring f198(long a0) { return std::to_wstring(a0); }

std::wstring f199(double a0) { return std::to_wstring(a0); }

std::wstring f200(float a0) { return std::to_wstring(a0); }

std::wstring f201(unsigned int a0) { return std::to_wstring(a0); }

std::wstring f202(unsigned long long a0) { return std::to_wstring(a0); }

std::wstring f203(unsigned long a0) { return std::to_wstring(a0); }
