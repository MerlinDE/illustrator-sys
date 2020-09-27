//
// Created by Alexander Czernay on 11.09.20.
//

#include "wrapper.h"
#include <stdio.h>

extern struct dummy unicode_string_from_utf8_impl(const char *string);

struct dummy unicode_string_from_utf8(const char *string) {
    printf("%s", string);
    dfd
    return unicode_string_from_utf8_impl(string);
};
