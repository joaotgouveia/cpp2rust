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

bool f16(const std::istrstream &a0) { return static_cast<bool>(a0); }

bool f17(const std::istrstream &a0) { return a0.operator!(); }

std::ios_base::iostate f18(const std::istrstream &a0) { return a0.rdstate(); }

void f19(std::istrstream &a0, std::ios_base::iostate a1) {
  return a0.clear(a1);
}

void f20(std::istrstream &a0, std::ios_base::iostate a1) {
  return a0.setstate(a1);
}

bool f21(const std::istrstream &a0) { return a0.good(); }

bool f22(const std::istrstream &a0) { return a0.eof(); }

bool f23(const std::istrstream &a0) { return a0.fail(); }

bool f24(const std::istrstream &a0) { return a0.bad(); }

std::ios_base::iostate f25(const std::istrstream &a0) {
  return a0.exceptions();
}

void f26(std::istrstream &a0, std::ios_base::iostate a1) {
  return a0.exceptions(a1);
}

std::basic_ostream<std::basic_ios<char, std::char_traits<char>>::char_type,
                   std::basic_ios<char, std::char_traits<char>>::traits_type> *
f27(const std::istrstream &a0) {
  return a0.tie();
}

std::basic_ostream<std::basic_ios<char, std::char_traits<char>>::char_type,
                   std::basic_ios<char, std::char_traits<char>>::traits_type> *
f28(std::istrstream &a0,
    std::basic_ostream<
        std::basic_ios<char, std::char_traits<char>>::char_type,
        std::basic_ios<char, std::char_traits<char>>::traits_type> *a1) {
  return a0.tie(a1);
}

std::basic_ios<char>::char_type f29(const std::istrstream &a0) {
  return a0.fill();
}

std::basic_ios<char>::char_type f30(std::istrstream &a0,
                                    std::basic_ios<char>::char_type a1) {
  return a0.fill(a1);
}

std::locale f31(std::istrstream &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

char f32(const std::istrstream &a0, std::basic_ios<char>::char_type a1,
         char a2) {
  return a0.narrow(a1, a2);
}

std::basic_ios<char>::char_type f33(const std::istrstream &a0, char a1) {
  return a0.widen(a1);
}

std::ios_base::fmtflags f34(const std::istrstream &a0) { return a0.flags(); }

std::ios_base::fmtflags f35(std::istrstream &a0, std::ios_base::fmtflags a1) {
  return a0.flags(a1);
}

std::ios_base::fmtflags f36(std::istrstream &a0, std::ios_base::fmtflags a1) {
  return a0.setf(a1);
}

std::ios_base::fmtflags f37(std::istrstream &a0, std::ios_base::fmtflags a1,
                            std::ios_base::fmtflags a2) {
  return a0.setf(a1, a2);
}

void f38(std::istrstream &a0, std::ios_base::fmtflags a1) {
  return a0.unsetf(a1);
}

std::streamsize f39(const std::istrstream &a0) { return a0.precision(); }

std::streamsize f40(std::istrstream &a0, std::streamsize a1) {
  return a0.precision(a1);
}

std::streamsize f41(const std::istrstream &a0) { return a0.width(); }

std::streamsize f42(std::istrstream &a0, std::streamsize a1) {
  return a0.width(a1);
}

std::locale f43(const std::istrstream &a0) { return a0.getloc(); }

int f44() { return std::ios_base::xalloc(); }

long &f45(std::istrstream &a0, int a1) { return a0.iword(a1); }

void *&f46(std::istrstream &a0, int a1) { return a0.pword(a1); }

void f47(std::istrstream &a0,
         void (*a1)(std::ios_base::event, std::ios_base &, int), int a2) {
  return a0.register_callback(a1, a2);
}

bool f48(bool a0) { return std::ios_base::sync_with_stdio(a0); }

using t2 = std::ostrstream;

std::ostrstream f49() { return std::ostrstream(); }

std::ostrstream f50(char *a0, int a1, std::ios_base::openmode a2) {
  return std::ostrstream(a0, a1, a2);
}

std::ostrstream f51(std::ostrstream &&a0) {
  return std::ostrstream(std::move(a0));
}

std::ostrstream &f52(std::ostrstream &a0, std::ostrstream &&a1) {
  return a0.operator=(std::move(a1));
}

void f53(std::ostrstream &a0, std::ostrstream &a1) { return a0.swap(a1); }

std::strstreambuf *f54(const std::ostrstream &a0) { return a0.rdbuf(); }

void f55(std::ostrstream &a0, bool a1) { return a0.freeze(a1); }

char *f56(std::ostrstream &a0) { return a0.str(); }

int f57(const std::ostrstream &a0) { return a0.pcount(); }

std::basic_ostream<char>::pos_type f58(std::ostrstream &a0) {
  return a0.tellp();
}

bool f59(const std::ostrstream &a0) { return static_cast<bool>(a0); }

bool f60(const std::ostrstream &a0) { return a0.operator!(); }

std::ios_base::iostate f61(const std::ostrstream &a0) { return a0.rdstate(); }

void f62(std::ostrstream &a0, std::ios_base::iostate a1) {
  return a0.clear(a1);
}

void f63(std::ostrstream &a0, std::ios_base::iostate a1) {
  return a0.setstate(a1);
}

bool f64(const std::ostrstream &a0) { return a0.good(); }

bool f65(const std::ostrstream &a0) { return a0.eof(); }

bool f66(const std::ostrstream &a0) { return a0.fail(); }

bool f67(const std::ostrstream &a0) { return a0.bad(); }

std::ios_base::iostate f68(const std::ostrstream &a0) {
  return a0.exceptions();
}

void f69(std::ostrstream &a0, std::ios_base::iostate a1) {
  return a0.exceptions(a1);
}

std::basic_ostream<std::basic_ios<char, std::char_traits<char>>::char_type,
                   std::basic_ios<char, std::char_traits<char>>::traits_type> *
f70(const std::ostrstream &a0) {
  return a0.tie();
}

std::basic_ostream<std::basic_ios<char, std::char_traits<char>>::char_type,
                   std::basic_ios<char, std::char_traits<char>>::traits_type> *
f71(std::ostrstream &a0,
    std::basic_ostream<
        std::basic_ios<char, std::char_traits<char>>::char_type,
        std::basic_ios<char, std::char_traits<char>>::traits_type> *a1) {
  return a0.tie(a1);
}

std::basic_ios<char>::char_type f72(const std::ostrstream &a0) {
  return a0.fill();
}

std::basic_ios<char>::char_type f73(std::ostrstream &a0,
                                    std::basic_ios<char>::char_type a1) {
  return a0.fill(a1);
}

std::locale f74(std::ostrstream &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

char f75(const std::ostrstream &a0, std::basic_ios<char>::char_type a1,
         char a2) {
  return a0.narrow(a1, a2);
}

std::basic_ios<char>::char_type f76(const std::ostrstream &a0, char a1) {
  return a0.widen(a1);
}

std::ios_base::fmtflags f77(const std::ostrstream &a0) { return a0.flags(); }

std::ios_base::fmtflags f78(std::ostrstream &a0, std::ios_base::fmtflags a1) {
  return a0.flags(a1);
}

std::ios_base::fmtflags f79(std::ostrstream &a0, std::ios_base::fmtflags a1) {
  return a0.setf(a1);
}

std::ios_base::fmtflags f80(std::ostrstream &a0, std::ios_base::fmtflags a1,
                            std::ios_base::fmtflags a2) {
  return a0.setf(a1, a2);
}

void f81(std::ostrstream &a0, std::ios_base::fmtflags a1) {
  return a0.unsetf(a1);
}

std::streamsize f82(const std::ostrstream &a0) { return a0.precision(); }

std::streamsize f83(std::ostrstream &a0, std::streamsize a1) {
  return a0.precision(a1);
}

std::streamsize f84(const std::ostrstream &a0) { return a0.width(); }

std::streamsize f85(std::ostrstream &a0, std::streamsize a1) {
  return a0.width(a1);
}

std::locale f86(const std::ostrstream &a0) { return a0.getloc(); }

long &f87(std::ostrstream &a0, int a1) { return a0.iword(a1); }

void *&f88(std::ostrstream &a0, int a1) { return a0.pword(a1); }

void f89(std::ostrstream &a0,
         void (*a1)(std::ios_base::event, std::ios_base &, int), int a2) {
  return a0.register_callback(a1, a2);
}

using t3 = std::strstream;

using t4 = std::strstream::char_type;

using t5 = std::strstream::int_type;

using t6 = std::strstream::pos_type;

using t7 = std::strstream::off_type;

std::strstream f90() { return std::strstream(); }

std::strstream f91(char *a0, int a1, std::ios_base::openmode a2) {
  return std::strstream(a0, a1, a2);
}

std::strstream f92(std::strstream &&a0) {
  return std::strstream(std::move(a0));
}

std::strstream &f93(std::strstream &a0, std::strstream &&a1) {
  return a0.operator=(std::move(a1));
}

void f94(std::strstream &a0, std::strstream &a1) { return a0.swap(a1); }

std::strstreambuf *f95(const std::strstream &a0) { return a0.rdbuf(); }

void f96(std::strstream &a0, bool a1) { return a0.freeze(a1); }

int f97(const std::strstream &a0) { return a0.pcount(); }

char *f98(std::strstream &a0) { return a0.str(); }

std::streamsize f99(const std::strstream &a0) { return a0.gcount(); }

std::basic_istream<char>::int_type f100(std::strstream &a0) { return a0.get(); }

std::basic_istream<char>::int_type f101(std::strstream &a0) {
  return a0.peek();
}

std::streamsize f102(std::strstream &a0,
                     std::basic_istream<char>::char_type *a1,
                     std::streamsize a2) {
  return a0.readsome(a1, a2);
}

int f103(std::strstream &a0) { return a0.sync(); }

std::basic_istream<char>::pos_type f104(std::strstream &a0) {
  return a0.tellg();
}

bool f105(const std::strstream &a0) { return static_cast<bool>(a0); }

bool f106(const std::strstream &a0) { return a0.operator!(); }

std::ios_base::iostate f107(const std::strstream &a0) { return a0.rdstate(); }

void f108(std::strstream &a0, std::ios_base::iostate a1) {
  return a0.clear(a1);
}

void f109(std::strstream &a0, std::ios_base::iostate a1) {
  return a0.setstate(a1);
}

bool f110(const std::strstream &a0) { return a0.good(); }

bool f111(const std::strstream &a0) { return a0.eof(); }

bool f112(const std::strstream &a0) { return a0.fail(); }

bool f113(const std::strstream &a0) { return a0.bad(); }

std::ios_base::iostate f114(const std::strstream &a0) {
  return a0.exceptions();
}

void f115(std::strstream &a0, std::ios_base::iostate a1) {
  return a0.exceptions(a1);
}

std::basic_ostream<std::basic_ios<char, std::char_traits<char>>::char_type,
                   std::basic_ios<char, std::char_traits<char>>::traits_type> *
f116(const std::strstream &a0) {
  return a0.tie();
}

std::basic_ostream<std::basic_ios<char, std::char_traits<char>>::char_type,
                   std::basic_ios<char, std::char_traits<char>>::traits_type> *
f117(std::strstream &a0,
     std::basic_ostream<
         std::basic_ios<char, std::char_traits<char>>::char_type,
         std::basic_ios<char, std::char_traits<char>>::traits_type> *a1) {
  return a0.tie(a1);
}

std::basic_ios<char>::char_type f118(const std::strstream &a0) {
  return a0.fill();
}

std::basic_ios<char>::char_type f119(std::strstream &a0,
                                     std::basic_ios<char>::char_type a1) {
  return a0.fill(a1);
}

std::locale f120(std::strstream &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

char f121(const std::strstream &a0, std::basic_ios<char>::char_type a1,
          char a2) {
  return a0.narrow(a1, a2);
}

std::basic_ios<char>::char_type f122(const std::strstream &a0, char a1) {
  return a0.widen(a1);
}

std::ios_base::fmtflags f123(const std::strstream &a0) { return a0.flags(); }

std::ios_base::fmtflags f124(std::strstream &a0, std::ios_base::fmtflags a1) {
  return a0.flags(a1);
}

std::ios_base::fmtflags f125(std::strstream &a0, std::ios_base::fmtflags a1) {
  return a0.setf(a1);
}

std::ios_base::fmtflags f126(std::strstream &a0, std::ios_base::fmtflags a1,
                             std::ios_base::fmtflags a2) {
  return a0.setf(a1, a2);
}

void f127(std::strstream &a0, std::ios_base::fmtflags a1) {
  return a0.unsetf(a1);
}

std::streamsize f128(const std::strstream &a0) { return a0.precision(); }

std::streamsize f129(std::strstream &a0, std::streamsize a1) {
  return a0.precision(a1);
}

std::streamsize f130(const std::strstream &a0) { return a0.width(); }

std::streamsize f131(std::strstream &a0, std::streamsize a1) {
  return a0.width(a1);
}

std::locale f132(const std::strstream &a0) { return a0.getloc(); }

long &f133(std::strstream &a0, int a1) { return a0.iword(a1); }

void *&f134(std::strstream &a0, int a1) { return a0.pword(a1); }

void f135(std::strstream &a0,
          void (*a1)(std::ios_base::event, std::ios_base &, int), int a2) {
  return a0.register_callback(a1, a2);
}

std::basic_ostream<char>::pos_type f136(std::strstream &a0) {
  return a0.tellp();
}

using t8 = std::strstreambuf;

std::strstreambuf f137() { return std::strstreambuf(); }

std::strstreambuf f138(std::streamsize a0) { return std::strstreambuf(a0); }

std::strstreambuf f139(void *(*a0)(std::size_t), void (*a1)(void *)) {
  return std::strstreambuf(a0, a1);
}

std::strstreambuf f140(char *a0, std::streamsize a1, char *a2) {
  return std::strstreambuf(a0, a1, a2);
}

std::strstreambuf f141(const char *a0, std::streamsize a1) {
  return std::strstreambuf(a0, a1);
}

std::strstreambuf f142(signed char *a0, std::streamsize a1, signed char *a2) {
  return std::strstreambuf(a0, a1, a2);
}

std::strstreambuf f143(const signed char *a0, std::streamsize a1) {
  return std::strstreambuf(a0, a1);
}

std::strstreambuf f144(unsigned char *a0, std::streamsize a1,
                       unsigned char *a2) {
  return std::strstreambuf(a0, a1, a2);
}

std::strstreambuf f145(const unsigned char *a0, std::streamsize a1) {
  return std::strstreambuf(a0, a1);
}

std::strstreambuf f146(std::strstreambuf &&a0) {
  return std::strstreambuf(std::move(a0));
}

std::strstreambuf &f147(std::strstreambuf &a0, std::strstreambuf &&a1) {
  return a0.operator=(std::move(a1));
}

void f148(std::strstreambuf &a0, std::strstreambuf &a1) { return a0.swap(a1); }

void f149(std::strstreambuf &a0, bool a1) { return a0.freeze(a1); }

char *f150(std::strstreambuf &a0) { return a0.str(); }

int f151(const std::strstreambuf &a0) { return a0.pcount(); }

std::locale f152(std::strstreambuf &a0, const std::locale &a1) {
  return a0.pubimbue(a1);
}

std::locale f153(const std::strstreambuf &a0) { return a0.getloc(); }

std::basic_streambuf<char>::pos_type
f154(std::strstreambuf &a0, std::basic_streambuf<char>::off_type a1,
     std::ios_base::seekdir a2, std::ios_base::openmode a3) {
  return a0.pubseekoff(a1, a2, a3);
}

std::basic_streambuf<char>::pos_type
f155(std::strstreambuf &a0, std::basic_streambuf<char>::pos_type a1,
     std::ios_base::openmode a2) {
  return a0.pubseekpos(std::move(a1), a2);
}

int f156(std::strstreambuf &a0) { return a0.pubsync(); }

std::streamsize f157(std::strstreambuf &a0) { return a0.in_avail(); }

std::basic_streambuf<char>::int_type f158(std::strstreambuf &a0) {
  return a0.snextc();
}

std::basic_streambuf<char>::int_type f159(std::strstreambuf &a0) {
  return a0.sbumpc();
}

std::basic_streambuf<char>::int_type f160(std::strstreambuf &a0) {
  return a0.sgetc();
}

std::streamsize f161(std::strstreambuf &a0,
                     std::basic_streambuf<char>::char_type *a1,
                     std::streamsize a2) {
  return a0.sgetn(a1, a2);
}

std::basic_streambuf<char>::int_type
f162(std::strstreambuf &a0, std::basic_streambuf<char>::char_type a1) {
  return a0.sputbackc(a1);
}

std::basic_streambuf<char>::int_type f163(std::strstreambuf &a0) {
  return a0.sungetc();
}

std::basic_streambuf<char>::int_type
f164(std::strstreambuf &a0, std::basic_streambuf<char>::char_type a1) {
  return a0.sputc(a1);
}

std::streamsize f165(std::strstreambuf &a0,
                     const std::basic_streambuf<char>::char_type *a1,
                     std::streamsize a2) {
  return a0.sputn(a1, a2);
}
