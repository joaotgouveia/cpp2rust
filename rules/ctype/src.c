// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <ctype.h>

int f1001(int a0) { return toupper(a0); }

int f1002(int a0) { return tolower(a0); }

int f1003(int a0) { return (isalnum)(a0); }

int f1004(int a0) { return (isalpha)(a0); }

int f1005(int a0) { return (isblank)(a0); }

int f1006(int a0) { return (iscntrl)(a0); }

int f1007(int a0) { return (isdigit)(a0); }

int f1008(int a0) { return (isgraph)(a0); }

int f1009(int a0) { return (islower)(a0); }

int f1010(int a0) { return (isprint)(a0); }

int f1011(int a0) { return (ispunct)(a0); }

int f1012(int a0) { return (isspace)(a0); }

int f1013(int a0) { return (isupper)(a0); }

int f1014(int a0) { return (isxdigit)(a0); }
