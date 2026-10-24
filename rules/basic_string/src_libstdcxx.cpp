#include <algorithm>
#include <initializer_list>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f146(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reference
f147(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reference
f148(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reference
f149(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reference
f150(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reference
f151(std::basic_string<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reference
f152(const std::basic_string<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reference
f153(std::basic_string<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reference
f154(const std::basic_string<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f155(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, T1 *a3,
     T1 *a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f156(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, const T1 *a3,
     const T1 *a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f157(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2,
     typename std::basic_string<T1, T2, T3>::iterator a3,
     typename std::basic_string<T1, T2, T3>::iterator a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f158(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2,
     typename std::basic_string<T1, T2, T3>::const_iterator a3,
     typename std::basic_string<T1, T2, T3>::const_iterator a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = std::_RequireInputIter<T4>>
std::basic_string<T1, T2, T3> f159(T4 a0, T4 a1, const T3 &a2) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = std::_RequireInputIter<T4>>
std::basic_string<T1, T2, T3> &f160(std::basic_string<T1, T2, T3> &a0, T4 a1,
                                    T4 a2) {
  return a0.template append<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = std::_RequireInputIter<T4>>
std::basic_string<T1, T2, T3> &f161(std::basic_string<T1, T2, T3> &a0, T4 a1,
                                    T4 a2) {
  return a0.template assign<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = std::_RequireInputIter<T4>>
typename std::basic_string<T1, T2, T3>::iterator
f162(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1, T4 a2, T4 a3) {
  return a0.template insert<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
using t23 = typename std::basic_string<T1, T2, T3>::iterator::iterator_type;

template <typename T1, typename T2, typename T3>
using t24 =
    typename std::basic_string<T1, T2, T3>::const_iterator::iterator_type;

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f163(std::basic_istream<T1, T2> &&a0,
                                 std::basic_string<T1, T2, T3> &a1) {
  return std::getline<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f164(std::basic_istream<T1, T2> &&a0,
                                 std::basic_string<T1, T2, T3> &a1, T1 a2) {
  return std::getline<T1, T2, T3>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f165(std::basic_istream<T1, T2> &a0,
                                 std::basic_string<T1, T2, T3> &a1) {
  return std::getline<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f166(std::basic_istream<T1, T2> &a0,
                                 std::basic_string<T1, T2, T3> &a1, T1 a2) {
  return std::getline<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f167(std::basic_string<T1, T2, T3> &&a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return std::operator+ <T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f168(std::basic_string<T1, T2, T3> &&a0,
                                   const T1 *a1) {
  return std::operator+ <T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f169(std::basic_string<T1, T2, T3> &&a0,
                                   std::basic_string<T1, T2, T3> &&a1) {
  return std::operator+ <T1, T2, T3>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f170(std::basic_string<T1, T2, T3> &&a0, T1 a1) {
  return std::operator+ <T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f171(const std::basic_string<T1, T2, T3> &a0,
                                   std::basic_string<T1, T2, T3> &&a1) {
  return std::operator+ <T1, T2, T3>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f172(const std::basic_string<T1, T2, T3> &a0,
                                   const T1 *a1) {
  return std::operator+ <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f173(const std::basic_string<T1, T2, T3> &a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return std::operator+ <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f174(const std::basic_string<T1, T2, T3> &a0,
                                   T1 a1) {
  return std::operator+ <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f175(const T1 *a0,
                                   std::basic_string<T1, T2, T3> &&a1) {
  return std::operator+ <T1, T2, T3>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f176(const T1 *a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return std::operator+ <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f177(T1 a0, std::basic_string<T1, T2, T3> &&a1) {
  return std::operator+ <T1, T2, T3>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f178(T1 a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return std::operator+ <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f179(std::basic_ostream<T1, T2> &a0,
                                 const std::basic_string<T1, T2, T3> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f180(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return std::operator== <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f181(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return std::operator== <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f182(std::basic_istream<T1, T2> &a0,
                                 std::basic_string<T1, T2, T3> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

double f183(const std::string &a0, std::size_t *a1) {
  return std::stod(a0, a1);
}

double f184(const std::wstring &a0, std::size_t *a1) {
  return std::stod(a0, a1);
}

float f185(const std::string &a0, std::size_t *a1) { return std::stof(a0, a1); }

float f186(const std::wstring &a0, std::size_t *a1) {
  return std::stof(a0, a1);
}

int f187(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoi(a0, a1, a2);
}

int f188(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoi(a0, a1, a2);
}

long f189(const std::string &a0, std::size_t *a1, int a2) {
  return std::stol(a0, a1, a2);
}

long f190(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stol(a0, a1, a2);
}

long double f191(const std::string &a0, std::size_t *a1) {
  return std::stold(a0, a1);
}

long double f192(const std::wstring &a0, std::size_t *a1) {
  return std::stold(a0, a1);
}

long long f193(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoll(a0, a1, a2);
}

long long f194(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoll(a0, a1, a2);
}

unsigned long f195(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoul(a0, a1, a2);
}

unsigned long f196(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoul(a0, a1, a2);
}

unsigned long long f197(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoull(a0, a1, a2);
}

unsigned long long f198(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoull(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
void f199(std::basic_string<T1, T2, T3> &a0,
          std::basic_string<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}

std::string f200(long double a0) { return std::to_string(a0); }

std::string f201(int a0) { return std::to_string(a0); }

std::string f202(long long a0) { return std::to_string(a0); }

std::string f203(long a0) { return std::to_string(a0); }

std::string f204(double a0) { return std::to_string(a0); }

std::string f205(float a0) { return std::to_string(a0); }

std::string f206(unsigned int a0) { return std::to_string(a0); }

std::string f207(unsigned long long a0) { return std::to_string(a0); }

std::string f208(unsigned long a0) { return std::to_string(a0); }

std::wstring f209(long double a0) { return std::to_wstring(a0); }

std::wstring f210(int a0) { return std::to_wstring(a0); }

std::wstring f211(long long a0) { return std::to_wstring(a0); }

std::wstring f212(long a0) { return std::to_wstring(a0); }

std::wstring f213(double a0) { return std::to_wstring(a0); }

std::wstring f214(float a0) { return std::to_wstring(a0); }

std::wstring f215(unsigned int a0) { return std::to_wstring(a0); }

std::wstring f216(unsigned long long a0) { return std::to_wstring(a0); }

std::wstring f217(unsigned long a0) { return std::to_wstring(a0); }
