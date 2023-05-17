/**
 * @file utils.c
 * @author Vincent Marias ~ @qtf0x
 * @date 05/16/2023
 * 
 * @copyright MIT License Copyright (c) 2023 Vincent Marias
 * 
 * @brief Implementations for utils.h functions.
 */

#include "utils.h"

#define EPSILON 1e-6f // small value used to compare floating-point numbers

bool rlFeq(const float a, const float b)
{
	return fabsf(a - b) < EPSILON;
}
