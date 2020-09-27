//
// Created by Alexander Czernay on 06.09.20.
//

//#include "wrapper.hpp"
#include "IAIUnicodeString.h"
#include <stdio.h>

extern "C" {

void *unicode_string_from_utf8(const char *string) {
    printf("%s", string);
    // Prevent UnicodeString destructor from being called
    // See https://stackoverflow.com/questions/9123022/preventing-a-destructor-from-running-in-c
    /*char buffer[sizeof(ai::UnicodeString) + alignof(ai::UnicodeString)];
    char* aligned_buffer = buffer + alignof(ai::UnicodeString) - reinterpret_cast<intptr_t>(buffer) % alignof(ai::UnicodeString);
    ai::UnicodeString* object = new (aligned_buffer) ai::UnicodeString;
    memcpy(object, ai::UnicodeString::FromUTF8(string), sizeof(ai::UnicodeString);*/
    static ai::UnicodeString tmp_unicode_string = ai::UnicodeString::FromUTF8(string);
    return reinterpret_cast<void *>(*reinterpret_cast<int *>(&tmp_unicode_string));
}

}
