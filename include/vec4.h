/**
 * @file vec4.h
 * @author Vincent Marias ~ @qtf0x
 * @date 05/24/2023
 * 
 * @copyright MIT License Copyright (c) 2023 Vincent Marias
 * 
 * @brief Declarations for 4D vector type.
 */

#ifndef RAYLA_VEC4_H
#define RAYLA_VEC4_H

// blurb for inclusion into C++ translation units
#ifdef __cplusplus
extern "C" {

// this header does (virtually) nothing in C++; don't need it
#else
#include <stdbool.h> // for bool

#endif // __cplusplus

typedef struct RLvec4 RLvec4;

/**
 * A 4D collection of floating-point numbers.
 */
struct RLvec4 {
	union {
		float x, r;
	};

	union {
		float y, g;
	};

	union {
		float z, b;
	};

	union {
		float w, a;
	};
};

/**
 * Initialize the elements of a 4D vector.
 * 
 * @param[out] v vector to be initialized
 * @param[in] x value of first element
 * @param[in] y value of second element
 * @param[in] z value of third element
 * @param[in] w value of fourth element
 * @return RLvec4* pass through [v] to allow function use in expressions
 */
RLvec4 *rlVec4Init4f(RLvec4 *const v, const float x, const float y,
		     const float z, const float w);

/**
 * Initialize the elements of a 4D vector to all have the same value.
 * 
 * @param[out] v vector to be initialized
 * @param[in] val value to populate all vector elements
 * @return RLvec4* pass through [v] to allow function use in expressions
 */
RLvec4 *rlVec4Init1f(RLvec4 *const v, const float val);

/**
 * Calculate the magnitude of a 4D vector.
 * 
 * @param[in] v vector whose magnitude is to be calculated
 * @return float magnitude of [v]
 */
float rlVec4Magnitude(const RLvec4 *const v);

/**
 * Compare 4D vectors for element-wise equality.
 * 
 * @param[in] v1 first vector to compare
 * @param[in] v2 second vector to compare
 * @return [true] vector elements ("close-enough" to) equal
 * @return [false] vector elements not equal
 */
bool rlVec4Equal(const RLvec4 *const v1, const RLvec4 *const v2);

/**
 * Negate a 4D vector.
 * 
 * @param[out] neg result
 * @param[in] v vector whose elements are to be negated
 * @return RLvec4* pass through [neg] to allow function use in expressions
 */
RLvec4 *rlVec4Negate(RLvec4 *const neg, const RLvec4 *const v);

/**
 * Calculate element-wise sum of 4D vectors.
 * 
 * @param[out] sum result
 * @param[in] augend first vector to add
 * @param[in] append second vector to add
 * @return RLvec4* pass through [sum] to allow function use in expressions
 */
RLvec4 *rlVec4Add(RLvec4 *const sum, const RLvec4 *const augend,
		  const RLvec4 *const append);

/**
 * Calculate element-wise difference of 4D vectors.
 * 
 * @param[out] diff result
 * @param[in] minuend vector to subtract from
 * @param[in] subtrahend vector to subtract
 * @return RLvec4* pass through [diff] to allow function use in expressions
 */
RLvec4 *rlVec4Subtract(RLvec4 *const diff, const RLvec4 *const minuend,
		       const RLvec4 *const subtrahend);

/**
 * Calculate element-wise multiplication of a 4D vector by a scalar.
 * 
 * @param[out] product result
 * @param[in] multiplier scalar to multiply by
 * @param[in] multiplicand vector to be multiplied
 * @return RLvec4* pass through [product] to allow function use in expressions
 */
RLvec4 *rlVec4Multiply(RLvec4 *const product, const float multiplier,
		       const RLvec4 *const multiplicand);

/**
 * Normalize a 4D vector. That is, construct a vector of magnitude 1 and in the 
 * direction of the given vector.
 * 
 * @param[out] norm result
 * @param[in] v vector to be normalized
 * @return RLvec4* pass through [norm] to allow function use in expressions
 */
RLvec4 *rlVec4Normalize(RLvec4 *const norm, const RLvec4 *const v);

/**
 * Determine whether a given 4D tuple is a vector in homogenous 
 * coordinates (has a W component of 0).
 * 
 * @param[in] tup the tuple to test
 * @return [true] the tuple is a vector, with a W component of 0
 * @return [false] the tuple is not a point
 */
bool rlIsHomoVector(const RLvec4 *const tup);

/**
 * Determine whether a given 4D tuple is a point in homogenous 
 * coordinates (has a W component of 1).
 * 
 * @param[in] tup the tuple to test
 * @return [true] the tuple is a point, with a W component of 1
 * @return [false] the tuple is not a point
 */
bool rlIsHomoPoint(const RLvec4 *const tup);

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // RAYLA_VEC4_H
