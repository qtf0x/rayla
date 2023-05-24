/**
 * @file vec2.h
 * @author Vincent Marias ~ @qtf0x
 * @date 05/24/2023
 * 
 * @copyright MIT License Copyright (c) 2023 Vincent Marias
 * 
 * @brief Declarations for 2D vector type.
 */

#ifndef RAYLA_VEC2_H
#define RAYLA_VEC2_H

// blurb for inclusion into C++ translation units
#ifdef __cplusplus
extern "C" {

// this header does (virtually) nothing in C++; don't need it
#else
#include <stdbool.h> // for bool

#endif // __cplusplus

typedef struct RLvec2 RLvec2;

/**
 * A 2D collection of floating-point numbers.
 */
struct RLvec2 {
	union {
		float first, x, r;
	};

	union {
		float second, y, g;
	};
};

/**
 * Initialize the elements of a 2D vector.
 * 
 * @param[out] v vector to be initialized
 * @param[in] x value of first element
 * @param[in] y value of second element
 * @return RLvec2* pass through [v] to allow function use in expressions
 */
RLvec2 *rlVec2Init2f(RLvec2 *const v, const float x, const float y);

/**
 * Initialize the elements of a 2D vector to all have the same value.
 * 
 * @param[out] v vector to be initialized
 * @param[in] val value to populate all vector elements
 * @return RLvec2* pass through [v] to allow function use in expressions
 */
RLvec2 *rlVec2Init1f(RLvec2 *const v, const float val);

/**
 * Calculate the magnitude of a 2D vector.
 * 
 * @param[in] v vector whose magnitude is to be calculated
 * @return float magnitude of [v]
 */
float rlVec2Magnitude(const RLvec2 *const v);

/**
 * Compare 2D vectors for element-wise equality.
 * 
 * @param[in] v1 first vector to compare
 * @param[in] v2 second vector to compare
 * @return [true] vector elements ("close-enough" to) equal
 * @return [false] vector elements not equal
 */
bool rlVec2Equal(const RLvec2 *const v1, const RLvec2 *const v2);

/**
 * Negate a 2D vector.
 * 
 * @param[out] neg result
 * @param[in] v vector whose elements are to be negated
 * @return RLvec2* pass through [neg] to allow function use in expressions
 */
RLvec2 *rlVec2Negate(RLvec2 *const neg, const RLvec2 *const v);

/**
 * Calculate element-wise sum of 2D vectors.
 * 
 * @param[out] sum result
 * @param[in] augend first vector to add
 * @param[in] append second vector to add
 * @return RLvec2* pass through [sum] to allow function use in expressions
 */
RLvec2 *rlVec2Add(RLvec2 *const sum, const RLvec2 *const augend,
		  const RLvec2 *const append);

/**
 * Calculate element-wise difference of 2D vectors.
 * 
 * @param[out] diff result
 * @param[in] minuend vector to subtract from
 * @param[in] subtrahend vector to subtract
 * @return RLvec2* pass through [diff] to allow function use in expressions
 */
RLvec2 *rlVec2Subtract(RLvec2 *const diff, const RLvec2 *const minuend,
		       const RLvec2 *const subtrahend);

/**
 * Calculate element-wise multiplication of a 2D vector by a scalar.
 * 
 * @param[out] product result
 * @param[in] multiplier scalar to multiply by
 * @param[in] multiplicand vector to be multiplied
 * @return RLvec2* pass through [product] to allow function use in expressions
 */
RLvec2 *rlVec2Multiply(RLvec2 *const product, const float multiplier,
		       const RLvec2 *const multiplicand);

/**
 * Normalize a 2D vector. That is, construct a vector of magnitude 1 and in the 
 * direction of the given vector.
 * 
 * @param[out] norm result
 * @param[in] v vector to be normalized
 * @return RLvec2* pass through [norm] to allow function use in expressions
 */
RLvec2 *rlVec2Normalize(RLvec2 *const norm, const RLvec2 *const v);

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // RAYLA_VEC2_H
