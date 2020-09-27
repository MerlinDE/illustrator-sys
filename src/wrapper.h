//
// Created by Alexander Czernay on 11.09.20.
//

#ifndef ILLUSTRATOR_WRAPPER_H
#define ILLUSTRATOR_WRAPPER_H

struct dummy {
    void *dummy_ptr;
};

struct dummy unicode_string_from_utf8(const char *string);

#endif //ILLUSTRATOR_WRAPPER_H
