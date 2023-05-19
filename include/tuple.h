/**
 * @file tuple.h
 * @author Vincent Marias ~ @qtf0x
 * @date 05/16/2023
 * 
 * @copyright MIT License Copyright (c) 2023 Vincent Marias
 * 
 * @brief Definition and functionality for a 4D tuple data structure.
 */

#ifndef RAYLA_TUPLE_H
#define RAYLA_TUPLE_H

#include <stdbool.h> // for bool

/**
 * @brief A 4D collection of floating-point numbers.
 */
struct RLtuple {
	// values for all four dimensions
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
 * @brief Determines whether a given tuple is a point.
 * 
 * @param tup the tuple to test
 * @return true the tuple is a point, with a W component of 1
 * @return false the tuple is not a point
 */
bool rlIsPoint(const struct RLtuple tup);

/**
 * @brief Determines whether a given tuple is a vector.
 * 
 * @param tup the tuple to test
 * @return true the tuple is a vector, with a W component of 0
 * @return false the tuple is not a point
 */
bool rlIsVector(const struct RLtuple tup);

/**
 * @brief Compares two tuples for element-wise equality.
 * 
 * @param a first tuple to compare
 * @param b second tuple to compare
 * @return true tuple elements "close-enough" to equal
 * @return false tuple elements not equal
 */
bool rlTeq(const struct RLtuple a, const struct RLtuple b);

/**
 * @brief Creates a tuple from given elements.
 * 
 * @param x first element of tuple
 * @param y second element of tuple
 * @param z third element of tuple
 * @param w fourth element of tuple
 * @return struct RLtuple tuple constructed from given elements
 */
struct RLtuple rlMakeTuple(const float x, const float y, const float z,
			   const float w);

/**
 * @brief Creates a tuple whose elements are all equal to the argument given.
 * 
 * @param x value for all elements of tuple
 * @return struct RLtuple tuple constructed from given element
 */
struct RLtuple rlMakeTuple1(const float x);

/**
 * @brief Creates a point tuple from given xyz elements (W component 1).
 * 
 * @param x first element of point
 * @param y second element of point
 * @param z third element of point
 * @return struct RLtuple point constructed from from given xyz elements and 
 * with a W component of 1
 */
struct RLtuple rlMakePoint(const float x, const float y, const float z);

/**
 * @brief Creates a point tuple whose xyz elements are all equal to the 
 * argument given.
 * 
 * @param x value for xyz elements of point
 * @return struct RLtuple point constructed from given element and with a W 
 * component of 1
 */
struct RLtuple rlMakePoint1(const float x);

/**
 * @brief Creates a vector tuple from given xyz elements (W component 0).
 * 
 * @param x first element of vector
 * @param y second element of vector
 * @param z third element of vector
 * @return struct RLtuple vector constructed from given xyz elements and 
 * with a W component of 0
 */
struct RLtuple rlMakeVector(const float x, const float y, const float z);

/**
 * @brief Creates a vector tuple whose xyz elements are all equal to the 
 * arguemtn given.
 * 
 * @param x value for xyz elements of point
 * @return struct RLtuple vector constructed from given element element and 
 * with a W component of 0
 */
struct RLtuple rlMakeVector1(const float x);

/**
 * @brief Calculates element-wise sum of two tuples.
 * 
 * @param a augend (first tuple to add)
 * @param b append (second tuple to add)
 * @return struct RLtuple element-wise sum of a and b
 */
struct RLtuple rlTadd(const struct RLtuple a, const struct RLtuple b);

/**
 * @brief Calculates element-wise difference of two tuples.
 * 
 * @param a minuend (tuple to subtract from)
 * @param b subtrahend (tuple to subtract)
 * @return struct RLtuple element-wise difference of a and b
 */
struct RLtuple rlTsub(const struct RLtuple a, const struct RLtuple b);

/**
 * @brief Negates a tuple.
 * 
 * @param tup tuple to negate
 * @return struct RLtuple all elements negated
 */
struct RLtuple rlTneg(const struct RLtuple tup);

/**
 * @brief Calculates element-wise multiplication of a tuple by a scalar.
 * 
 * @param scl multiplier
 * @param tup multiplicand
 * @return struct RLtuple element-wise product of tup by scl
 */
struct RLtuple rlFTmul(const float scl, const struct RLtuple tup);

/**
 * @brief Computes the magnitude of a given vector.
 * 
 * @param v vector to find the magnitude of
 * @return float magnitude of the vector (0 if v is a point)
 */
float rlTmag(const struct RLtuple v);

#endif // RAYLA_TUPLE_H
