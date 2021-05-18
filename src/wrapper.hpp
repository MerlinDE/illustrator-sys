//
// Created by Alexander Czernay on 06.09.20.
//

#ifndef ILLUSTRATOR_WRAPPER_HPP
#define ILLUSTRATOR_WRAPPER_HPP

#include "IAIUnicodeString.h"
#include "IAIArtboards.hpp"

extern "C" void * /*ai::UnicodeString*/ unicode_string_from_utf8(const char *string);

#endif //ILLUSTRATOR_WRAPPER_HPP
