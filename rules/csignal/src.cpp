#include "../common/init.h"
#include "../common/traits.h"
#include <csignal>
#include <cstdint>
#include <stdlib.h>
#include <utility>

using t1 = fpregset_t;

using t2 = greg_t;

using t3 = gregset_t;

int f1(int a0) { return gsignal(a0); }

int f2(int a0, int a1) { return kill(a0, a1); }

int f3(int a0, int a1) { return killpg(a0, a1); }

using t4 = mcontext_t;

void f4(const siginfo_t *a0, const char *a1) { return psiginfo(a0, a1); }

void f5(int a0, const char *a1) { return psignal(a0, a1); }

int f6(pthread_t a0, int a1) { return pthread_kill(a0, a1); }

int f7(pthread_t a0, int a1, const union sigval a2) {
  return pthread_sigqueue(a0, a1, std::move(a2));
}

int f8(int a0) { return raise(a0); }

using t5 = sig_atomic_t;

using t6 = sig_t;

int f9(int a0, const struct sigaction *__restrict a1,
       struct sigaction *__restrict a2) {
  return sigaction(a0, a1, a2);
}

using t7 = struct sigaction;

int f10(sigset_t *a0, int a1) { return sigaddset(a0, a1); }

int f11(const stack_t *__restrict a0, stack_t *__restrict a1) {
  return sigaltstack(a0, a1);
}

int f12(sigset_t *a0, const sigset_t *a1, const sigset_t *a2) {
  return sigandset(a0, a1, a2);
}

using t8 = struct sigcontext;

int f13(sigset_t *a0, int a1) { return sigdelset(a0, a1); }

int f14(sigset_t *a0) { return sigemptyset(a0); }

using t9 = struct sigevent;

using t10 = sigevent_t;

int f15(sigset_t *a0) { return sigfillset(a0); }

using t11 = sighandler_t;

using t12 = siginfo_t;

int f16(const sigset_t *a0) { return sigisemptyset(a0); }

int f17(const sigset_t *a0, int a1) { return sigismember(a0, a1); }

void (*f18(int a0, void (*a1)(int)))(int) { return signal(a0, a1); }

int f19(sigset_t *a0, const sigset_t *a1, const sigset_t *a2) {
  return sigorset(a0, a1, a2);
}

int f20(sigset_t *a0) { return sigpending(a0); }

int f21(int a0, const sigset_t *__restrict a1, sigset_t *__restrict a2) {
  return sigprocmask(a0, a1, a2);
}

int f22(int a0, int a1, const union sigval a2) {
  return sigqueue(a0, a1, std::move(a2));
}

int f23(struct sigcontext *a0) { return sigreturn(a0); }

using t13 = struct sigstack;

int f24(const sigset_t *a0) { return sigsuspend(a0); }

int f25(const sigset_t *__restrict a0, siginfo_t *__restrict a1,
        const struct timespec *__restrict a2) {
  return sigtimedwait(a0, a1, a2);
}

using t14 = union sigval;

using t15 = sigval_t;

int f26(const sigset_t *__restrict a0, int *__restrict a1) {
  return sigwait(a0, a1);
}

int f27(const sigset_t *__restrict a0, siginfo_t *__restrict a1) {
  return sigwaitinfo(a0, a1);
}

void (*f28(int a0, void (*a1)(int)))(int) { return ssignal(a0, a1); }

using t16 = stack_t;

void (*f29(int a0, void (*a1)(int)))(int) { return sysv_signal(a0, a1); }

int f30(int a0, int a1, int a2) { return tgkill(a0, a1, a2); }

using t17 = struct ucontext_t;

using t18 = ucontext_t;
