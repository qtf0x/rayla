/**
 * @file utils.h
 * @author Vincent Marias ~ @qtf0x
 * @date 05/16/2023
 * 
 * @copyright MIT License Copyright (c) 2023 Vincent Marias
 * 
 * @brief Macros and utility functions for the Rayla renderer.
 */

#ifndef RAYLA_UTILS_H
#define RAYLA_UTILS_H

// blurb for inclusion into C++ translation units
#ifdef __cplusplus
extern "C" {

// this header does (virtually) nothing in C++; don't need it
#else
#include <stdbool.h> // for bool

#endif // __cplusplus

/**
 * @brief Compares two floating-point values for equality.
 * 
 * @param a first value to compare
 * @param b second value to compare
 * @return true values "close-enough" to equal
 * @return false values not equal
 */
bool feq(const float a, const float b);

#ifdef __cplusplus
}
#endif // __cplusplus

#endif //RAYLA_UTILS_H
