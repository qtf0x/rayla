/**
 * @file vec3.h
 * @author Vincent Marias ~ @qtf0x
 * @date 05/24/2023
 * 
 * @copyright MIT License Copyright (c) 2023 Vincent Marias
 * 
 * @brief Declarations for 3D vector type.
 */

#ifndef RAYLA_VEC3_H
#define RAYLA_VEC3_H

// blurb for inclusion into C++ translation units
#ifdef __cplusplus
extern "C" {

// this header does (virtually) nothing in C++; don't need it
#else
#include <stdbool.h> // for bool

#endif // __cplusplus

typedef struct RLvec3 RLvec3;

/**
 * A 3D collection of floating-point numbers.
 */
struct RLvec3 {
	union {
		float x, r;
	};

	union {
		float y, g;
	};

	union {
		float z, b;
	};
};

/**
 * Initialize the elements of a 3D vector.
 * 
 * @param[out] v vector to be initialized
 * @param[in] x value of first element
 * @param[in] y value of second element
 * @param[in] z value of third element
 * @return RLvec3* pass through [v] to allow function use in expressions
 */
RLvec3 *rlVec3Init3f(RLvec3 *const v, const float x, const float y,
		     const float z);

/**
 * Initialize the elements of a 3D vector to all have the same value.
 * 
 * @param[out] v vector to be initialized
 * @param[in] val value to populate all vector elements
 * @return RLvec3* pass through [v] to allow function use in expressions
 */
RLvec3 *rlVec3Init1f(RLvec3 *const v, const float val);

/**
 * Calculate the magnitude of a 3D vector.
 * 
 * @param[in] v vector whose magnitude is to be calculated
 * @return float magnitude of [v]
 */
float rlVec3Magnitude(const RLvec3 *const v);

/**
 * Compare 3D vectors for element-wise equality.
 * 
 * @param[in] v1 first vector to compare
 * @param[in] v2 second vector to compare
 * @return [true] vector elements ("close-enough" to) equal
 * @return [false] vector elements not equal
 */
bool rlVec3Equal(const RLvec3 *const v1, const RLvec3 *const v2);

/**
 * Negate a 3D vector.
 * 
 * @param[out] neg result
 * @param[in] v vector whose elements are to be negated
 * @return RLvec3* pass through [neg] to allow function use in expressions
 */
RLvec3 *rlVec3Negate(RLvec3 *const neg, const RLvec3 *const v);

/**
 * Calculate element-wise sum of 3D vectors.
 * 
 * @param[out] sum result
 * @param[in] augend first vector to add
 * @param[in] append second vector to add
 * @return RLvec3* pass through [sum] to allow function use in expressions
 */
RLvec3 *rlVec3Add(RLvec3 *const sum, const RLvec3 *const augend,
		  const RLvec3 *const append);

/**
 * Calculate element-wise difference of 3D vectors.
 * 
 * @param[out] diff result
 * @param[in] minuend vector to subtract from
 * @param[in] subtrahend vector to subtract
 * @return RLvec3* pass through [diff] to allow function use in expressions
 */
RLvec3 *rlVec3Subtract(RLvec3 *const diff, const RLvec3 *const minuend,
		       const RLvec3 *const subtrahend);

/**
 * Calculate element-wise multiplication of a 3D vector by a scalar.
 * 
 * @param[out] product result
 * @param[in] multiplier scalar to multiply by
 * @param[in] multiplicand vector to be multiplied
 * @return RLvec3* pass through [product] to allow function use in expressions
 */
RLvec3 *rlVec3Multiply(RLvec3 *const product, const float multiplier,
		       const RLvec3 *const multiplicand);

/**
 * Normalize a 3D vector. That is, construct a vector of magnitude 1 and in the 
 * direction of the given vector.
 * 
 * @param[out] norm result
 * @param[in] v vector to be normalized
 * @return RLvec3* pass through [norm] to allow function use in expressions
 */
RLvec3 *rlVec3Normalize(RLvec3 *const norm, const RLvec3 *const v);

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // RAYLA_VEC3_H
