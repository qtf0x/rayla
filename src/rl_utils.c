/**
 * @file utils.c
 * @author Vincent Marias ~ @qtf0x
 * @date 05/16/2023
 * 
 * @copyright MIT License Copyright (c) 2023 Vincent Marias
 * 
 * @brief Implementations for utils.h functions.
 */

#include "rl_utils.h"

#include <math.h> // for fabsf

// small value used to compare floating-point numbers
static const float EPSILON = 1e-6f;

bool feq(const float a, const float b)
{
	return fabsf(a - b) < EPSILON;
}
