#include <netinet/in.h>
#include <netinet/tcp.h>

typedef struct in_addr t2;

int f1(void) { return IPPROTO_TCP; }

int f2(void) { return IPPROTO_UDP; }

int f3(void) { return IPPROTO_IP; }

int f4(void) { return IPPROTO_IPV6; }

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-stack-address"
#pragma clang diagnostic pop
