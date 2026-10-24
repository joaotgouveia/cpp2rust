#include <cstddef>
#include <ios>
#include <iosfwd>
#include <istream>
#include <ostream>
#include <streambuf>
#include <strstream>
#include <utility>

using t1 = std::istrstream;

std::istrstream f1(const char *a0) { return std::istrstream(a0); }

std::istrstream f2(char *a0) { return std::istrstream(a0); }

std::istrstream f3(const char *a0, std::streamsize a1) {
  return std::istrstream(a0, a1);
}

std::istrstream f4(char *a0, std::streamsize a1) {
  return std::istrstream(a0, a1);
}

std::istrstream f5(std::istrstream &&a0) {
  return std::istrstream(std::move(a0));
}

std::istrstream &f6(std::istrstream &a0, std::istrstream &&a1) {
  return a0.operator=(std::move(a1));
}

void f7(std::istrstream &a0, std::istrstream &a1) { return a0.swap(a1); }

std::strstreambuf *f8(const std::istrstream &a0) { return a0.rdbuf(); }

char *f9(std::istrstream &a0) { return a0.str(); }

std::streamsize f10(const std::istrstream &a0) { return a0.gcount(); }

std::basic_istream<char>::int_type f11(std::istrstream &a0) { return a0.get(); }

std::basic_istream<char>::int_type f12(std::istrstream &a0) {
  return a0.peek();
}

std::streamsize f13(std::istrstream &a0,
                    std::basic_istream<char>::char_type *a1,
                    std::streamsize a2) {
  return a0.readsome(a1, a2);
}

int f14(std::istrstream &a0) { return a0.sync(); }

std::basic_istream<char>::pos_type f15(std::istrstream &a0) {
  return a0.tellg();
}

template <
    typename T1 = char,
    std::__enable_if_t<std::integral_constant<bool, true>::value, int> T2 = 0>
std::istream &f16(std::istrstream &a0, std::streamsize a1, char a2) {
  return a0.ignore(a1, a2);
}

bool f17(const std::istrstream &a0) { return static_cast<bool>(a0); }

bool f18(const std::istrstream &a0) { return a0.operator!(); }

std::ios_base::iostate f19(const std::istrstream &a0) { return a0.rdstate(); }

void f20(std::istrstream &a0, std::ios_base::iostate a1) {
  return a0.clear(a1);
}

void f21(std::istrstream &a0, std::ios_base::iostate a1) {
  return a0.setstate(a1);
}

bool f22(const std::istrstream &a0) { return a0.good(); }

bool f23(const std::istrstream &a0) { return a0.eof(); }

bool f24(const std::istrstream &a0) { return a0.fail(); }

bool f25(const std::istrstream &a0) { return a0.bad(); }

std::ios_base::iostate f26(const std::istrstream &a0) {
  return a0.exceptions();
}

void f27(std::istrstream &a0, std::ios_base::iostate a1) {
  return a0.exceptions(a1);
}

std::basic_ostream<std::basic_ios<char, std::char_traits<char>>::char_type,
                   std::basic_ios<char, std::char_traits<char>>::traits_type> *
f28(const std::istrstream &a0) {
  return a0.tie();
}

std::basic_ostream<std::basic_ios<char, std::char_traits<char>>::char_type,
                   std::basic_ios<char, std::char_traits<char>>::traits_type> *
f29(std::istrstream &a0,
    std::basic_ostream<
        std::basic_ios<char, std::char_traits<char>>::char_type,
        std::basic_ios<char, std::char_traits<char>>::traits_type> *a1) {
  return a0.tie(a1);
}

std::basic_ios<char>::char_type f30(const std::istrstream &a0) {
  return a0.fill();
}

std::basic_ios<char>::char_type f31(std::istrstream &a0,
                                    std::basic_ios<char>::char_type a1) {
  return a0.fill(a1);
}

std::locale f32(std::istrstream &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

char f33(const std::istrstream &a0, std::basic_ios<char>::char_type a1,
         char a2) {
  return a0.narrow(a1, a2);
}

std::basic_ios<char>::char_type f34(const std::istrstream &a0, char a1) {
  return a0.widen(a1);
}

std::ios_base::fmtflags f35(const std::istrstream &a0) { return a0.flags(); }

std::ios_base::fmtflags f36(std::istrstream &a0, std::ios_base::fmtflags a1) {
  return a0.flags(a1);
}

std::ios_base::fmtflags f37(std::istrstream &a0, std::ios_base::fmtflags a1) {
  return a0.setf(a1);
}

std::ios_base::fmtflags f38(std::istrstream &a0, std::ios_base::fmtflags a1,
                            std::ios_base::fmtflags a2) {
  return a0.setf(a1, a2);
}

void f39(std::istrstream &a0, std::ios_base::fmtflags a1) {
  return a0.unsetf(a1);
}

std::streamsize f40(const std::istrstream &a0) { return a0.precision(); }

std::streamsize f41(std::istrstream &a0, std::streamsize a1) {
  return a0.precision(a1);
}

std::streamsize f42(const std::istrstream &a0) { return a0.width(); }

std::streamsize f43(std::istrstream &a0, std::streamsize a1) {
  return a0.width(a1);
}

std::locale f44(const std::istrstream &a0) { return a0.getloc(); }

int f45() { return std::istrstream::xalloc(); }

long &f46(std::istrstream &a0, int a1) { return a0.iword(a1); }

void *&f47(std::istrstream &a0, int a1) { return a0.pword(a1); }

void f48(std::istrstream &a0,
         void (*a1)(std::ios_base::event, std::ios_base &, int), int a2) {
  return a0.register_callback(a1, a2);
}

bool f49(bool a0) { return std::istrstream::sync_with_stdio(a0); }

using t2 = std::ostrstream;

std::ostrstream f50() { return std::ostrstream(); }

std::ostrstream f51(char *a0, int a1, std::ios_base::openmode a2) {
  return std::ostrstream(a0, a1, a2);
}

std::ostrstream f52(std::ostrstream &&a0) {
  return std::ostrstream(std::move(a0));
}

std::ostrstream &f53(std::ostrstream &a0, std::ostrstream &&a1) {
  return a0.operator=(std::move(a1));
}

void f54(std::ostrstream &a0, std::ostrstream &a1) { return a0.swap(a1); }

std::strstreambuf *f55(const std::ostrstream &a0) { return a0.rdbuf(); }

void f56(std::ostrstream &a0, bool a1) { return a0.freeze(a1); }

char *f57(std::ostrstream &a0) { return a0.str(); }

int f58(const std::ostrstream &a0) { return a0.pcount(); }

std::basic_ostream<char>::pos_type f59(std::ostrstream &a0) {
  return a0.tellp();
}

bool f60(const std::ostrstream &a0) { return static_cast<bool>(a0); }

bool f61(const std::ostrstream &a0) { return a0.operator!(); }

std::ios_base::iostate f62(const std::ostrstream &a0) { return a0.rdstate(); }

void f63(std::ostrstream &a0, std::ios_base::iostate a1) {
  return a0.clear(a1);
}

void f64(std::ostrstream &a0, std::ios_base::iostate a1) {
  return a0.setstate(a1);
}

bool f65(const std::ostrstream &a0) { return a0.good(); }

bool f66(const std::ostrstream &a0) { return a0.eof(); }

bool f67(const std::ostrstream &a0) { return a0.fail(); }

bool f68(const std::ostrstream &a0) { return a0.bad(); }

std::ios_base::iostate f69(const std::ostrstream &a0) {
  return a0.exceptions();
}

void f70(std::ostrstream &a0, std::ios_base::iostate a1) {
  return a0.exceptions(a1);
}

std::basic_ostream<std::basic_ios<char, std::char_traits<char>>::char_type,
                   std::basic_ios<char, std::char_traits<char>>::traits_type> *
f71(const std::ostrstream &a0) {
  return a0.tie();
}

std::basic_ostream<std::basic_ios<char, std::char_traits<char>>::char_type,
                   std::basic_ios<char, std::char_traits<char>>::traits_type> *
f72(std::ostrstream &a0,
    std::basic_ostream<
        std::basic_ios<char, std::char_traits<char>>::char_type,
        std::basic_ios<char, std::char_traits<char>>::traits_type> *a1) {
  return a0.tie(a1);
}

std::basic_ios<char>::char_type f73(const std::ostrstream &a0) {
  return a0.fill();
}

std::basic_ios<char>::char_type f74(std::ostrstream &a0,
                                    std::basic_ios<char>::char_type a1) {
  return a0.fill(a1);
}

std::locale f75(std::ostrstream &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

char f76(const std::ostrstream &a0, std::basic_ios<char>::char_type a1,
         char a2) {
  return a0.narrow(a1, a2);
}

std::basic_ios<char>::char_type f77(const std::ostrstream &a0, char a1) {
  return a0.widen(a1);
}

std::ios_base::fmtflags f78(const std::ostrstream &a0) { return a0.flags(); }

std::ios_base::fmtflags f79(std::ostrstream &a0, std::ios_base::fmtflags a1) {
  return a0.flags(a1);
}

std::ios_base::fmtflags f80(std::ostrstream &a0, std::ios_base::fmtflags a1) {
  return a0.setf(a1);
}

std::ios_base::fmtflags f81(std::ostrstream &a0, std::ios_base::fmtflags a1,
                            std::ios_base::fmtflags a2) {
  return a0.setf(a1, a2);
}

void f82(std::ostrstream &a0, std::ios_base::fmtflags a1) {
  return a0.unsetf(a1);
}

std::streamsize f83(const std::ostrstream &a0) { return a0.precision(); }

std::streamsize f84(std::ostrstream &a0, std::streamsize a1) {
  return a0.precision(a1);
}

std::streamsize f85(const std::ostrstream &a0) { return a0.width(); }

std::streamsize f86(std::ostrstream &a0, std::streamsize a1) {
  return a0.width(a1);
}

std::locale f87(const std::ostrstream &a0) { return a0.getloc(); }

int f88() { return std::ostrstream::xalloc(); }

long &f89(std::ostrstream &a0, int a1) { return a0.iword(a1); }

void *&f90(std::ostrstream &a0, int a1) { return a0.pword(a1); }

void f91(std::ostrstream &a0,
         void (*a1)(std::ios_base::event, std::ios_base &, int), int a2) {
  return a0.register_callback(a1, a2);
}

bool f92(bool a0) { return std::ostrstream::sync_with_stdio(a0); }

using t3 = std::strstream;

using t4 = std::strstream::char_type;

using t5 = std::strstream::int_type;

using t6 = std::strstream::pos_type;

using t7 = std::strstream::off_type;

std::strstream f93() { return std::strstream(); }

std::strstream f94(char *a0, int a1, std::ios_base::openmode a2) {
  return std::strstream(a0, a1, a2);
}

std::strstream f95(std::strstream &&a0) {
  return std::strstream(std::move(a0));
}

std::strstream &f96(std::strstream &a0, std::strstream &&a1) {
  return a0.operator=(std::move(a1));
}

void f97(std::strstream &a0, std::strstream &a1) { return a0.swap(a1); }

std::strstreambuf *f98(const std::strstream &a0) { return a0.rdbuf(); }

void f99(std::strstream &a0, bool a1) { return a0.freeze(a1); }

int f100(const std::strstream &a0) { return a0.pcount(); }

char *f101(std::strstream &a0) { return a0.str(); }

std::streamsize f102(const std::strstream &a0) { return a0.gcount(); }

std::basic_istream<char>::int_type f103(std::strstream &a0) { return a0.get(); }

std::basic_istream<char>::int_type f104(std::strstream &a0) {
  return a0.peek();
}

std::streamsize f105(std::strstream &a0,
                     std::basic_istream<char>::char_type *a1,
                     std::streamsize a2) {
  return a0.readsome(a1, a2);
}

int f106(std::strstream &a0) { return a0.sync(); }

std::basic_istream<char>::pos_type f107(std::strstream &a0) {
  return a0.tellg();
}

template <
    typename T1 = char,
    std::__enable_if_t<std::integral_constant<bool, true>::value, int> T2 = 0>
std::istream &f108(std::strstream &a0, std::streamsize a1, char a2) {
  return a0.ignore(a1, a2);
}

bool f109(const std::strstream &a0) { return static_cast<bool>(a0); }

bool f110(const std::strstream &a0) { return a0.operator!(); }

std::ios_base::iostate f111(const std::strstream &a0) { return a0.rdstate(); }

void f112(std::strstream &a0, std::ios_base::iostate a1) {
  return a0.clear(a1);
}

void f113(std::strstream &a0, std::ios_base::iostate a1) {
  return a0.setstate(a1);
}

bool f114(const std::strstream &a0) { return a0.good(); }

bool f115(const std::strstream &a0) { return a0.eof(); }

bool f116(const std::strstream &a0) { return a0.fail(); }

bool f117(const std::strstream &a0) { return a0.bad(); }

std::ios_base::iostate f118(const std::strstream &a0) {
  return a0.exceptions();
}

void f119(std::strstream &a0, std::ios_base::iostate a1) {
  return a0.exceptions(a1);
}

std::basic_ostream<std::basic_ios<char, std::char_traits<char>>::char_type,
                   std::basic_ios<char, std::char_traits<char>>::traits_type> *
f120(const std::strstream &a0) {
  return a0.tie();
}

std::basic_ostream<std::basic_ios<char, std::char_traits<char>>::char_type,
                   std::basic_ios<char, std::char_traits<char>>::traits_type> *
f121(std::strstream &a0,
     std::basic_ostream<
         std::basic_ios<char, std::char_traits<char>>::char_type,
         std::basic_ios<char, std::char_traits<char>>::traits_type> *a1) {
  return a0.tie(a1);
}

std::basic_ios<char>::char_type f122(const std::strstream &a0) {
  return a0.fill();
}

std::basic_ios<char>::char_type f123(std::strstream &a0,
                                     std::basic_ios<char>::char_type a1) {
  return a0.fill(a1);
}

std::locale f124(std::strstream &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

char f125(const std::strstream &a0, std::basic_ios<char>::char_type a1,
          char a2) {
  return a0.narrow(a1, a2);
}

std::basic_ios<char>::char_type f126(const std::strstream &a0, char a1) {
  return a0.widen(a1);
}

std::ios_base::fmtflags f127(const std::strstream &a0) { return a0.flags(); }

std::ios_base::fmtflags f128(std::strstream &a0, std::ios_base::fmtflags a1) {
  return a0.flags(a1);
}

std::ios_base::fmtflags f129(std::strstream &a0, std::ios_base::fmtflags a1) {
  return a0.setf(a1);
}

std::ios_base::fmtflags f130(std::strstream &a0, std::ios_base::fmtflags a1,
                             std::ios_base::fmtflags a2) {
  return a0.setf(a1, a2);
}

void f131(std::strstream &a0, std::ios_base::fmtflags a1) {
  return a0.unsetf(a1);
}

std::streamsize f132(const std::strstream &a0) { return a0.precision(); }

std::streamsize f133(std::strstream &a0, std::streamsize a1) {
  return a0.precision(a1);
}

std::streamsize f134(const std::strstream &a0) { return a0.width(); }

std::streamsize f135(std::strstream &a0, std::streamsize a1) {
  return a0.width(a1);
}

std::locale f136(const std::strstream &a0) { return a0.getloc(); }

int f137() { return std::strstream::xalloc(); }

long &f138(std::strstream &a0, int a1) { return a0.iword(a1); }

void *&f139(std::strstream &a0, int a1) { return a0.pword(a1); }

void f140(std::strstream &a0,
          void (*a1)(std::ios_base::event, std::ios_base &, int), int a2) {
  return a0.register_callback(a1, a2);
}

bool f141(bool a0) { return std::strstream::sync_with_stdio(a0); }

std::basic_ostream<char>::pos_type f142(std::strstream &a0) {
  return a0.tellp();
}

using t8 = std::strstreambuf;

std::strstreambuf f143() { return std::strstreambuf(); }

std::strstreambuf f144(std::streamsize a0) { return std::strstreambuf(a0); }

std::strstreambuf f145(void *(*a0)(std::size_t), void (*a1)(void *)) {
  return std::strstreambuf(a0, a1);
}

std::strstreambuf f146(char *a0, std::streamsize a1, char *a2) {
  return std::strstreambuf(a0, a1, a2);
}

std::strstreambuf f147(const char *a0, std::streamsize a1) {
  return std::strstreambuf(a0, a1);
}

std::strstreambuf f148(signed char *a0, std::streamsize a1, signed char *a2) {
  return std::strstreambuf(a0, a1, a2);
}

std::strstreambuf f149(const signed char *a0, std::streamsize a1) {
  return std::strstreambuf(a0, a1);
}

std::strstreambuf f150(unsigned char *a0, std::streamsize a1,
                       unsigned char *a2) {
  return std::strstreambuf(a0, a1, a2);
}

std::strstreambuf f151(const unsigned char *a0, std::streamsize a1) {
  return std::strstreambuf(a0, a1);
}

std::strstreambuf f152(std::strstreambuf &&a0) {
  return std::strstreambuf(std::move(a0));
}

std::strstreambuf &f153(std::strstreambuf &a0, std::strstreambuf &&a1) {
  return a0.operator=(std::move(a1));
}

void f154(std::strstreambuf &a0, std::strstreambuf &a1) { return a0.swap(a1); }

void f155(std::strstreambuf &a0, bool a1) { return a0.freeze(a1); }

char *f156(std::strstreambuf &a0) { return a0.str(); }

int f157(const std::strstreambuf &a0) { return a0.pcount(); }

std::locale f158(std::strstreambuf &a0, const std::locale &a1) {
  return a0.pubimbue(a1);
}

std::locale f159(const std::strstreambuf &a0) { return a0.getloc(); }

std::basic_streambuf<char>::pos_type
f160(std::strstreambuf &a0, std::basic_streambuf<char>::off_type a1,
     std::ios_base::seekdir a2, std::ios_base::openmode a3) {
  return a0.pubseekoff(a1, a2, a3);
}

std::basic_streambuf<char>::pos_type
f161(std::strstreambuf &a0, std::basic_streambuf<char>::pos_type a1,
     std::ios_base::openmode a2) {
  return a0.pubseekpos(std::move(a1), a2);
}

int f162(std::strstreambuf &a0) { return a0.pubsync(); }

std::streamsize f163(std::strstreambuf &a0) { return a0.in_avail(); }

std::basic_streambuf<char>::int_type f164(std::strstreambuf &a0) {
  return a0.snextc();
}

std::basic_streambuf<char>::int_type f165(std::strstreambuf &a0) {
  return a0.sbumpc();
}

std::basic_streambuf<char>::int_type f166(std::strstreambuf &a0) {
  return a0.sgetc();
}

std::streamsize f167(std::strstreambuf &a0,
                     std::basic_streambuf<char>::char_type *a1,
                     std::streamsize a2) {
  return a0.sgetn(a1, a2);
}

std::basic_streambuf<char>::int_type
f168(std::strstreambuf &a0, std::basic_streambuf<char>::char_type a1) {
  return a0.sputbackc(a1);
}

std::basic_streambuf<char>::int_type f169(std::strstreambuf &a0) {
  return a0.sungetc();
}

std::basic_streambuf<char>::int_type
f170(std::strstreambuf &a0, std::basic_streambuf<char>::char_type a1) {
  return a0.sputc(a1);
}

std::streamsize f171(std::strstreambuf &a0,
                     const std::basic_streambuf<char>::char_type *a1,
                     std::streamsize a2) {
  return a0.sputn(a1, a2);
}
