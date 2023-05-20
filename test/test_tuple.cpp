#include <cmath> // for sqrtf

#include <gtest/gtest.h>

extern "C" {
#include "tuple.h"
#include "rl_utils.h"
}

// Scenario: A tuple with w=1.0 is a point
TEST(tuple, wOnePoint)
{
	// Given a <- tuple(4.3, -4.2, 3.1, 1.0)
	auto a{ rlMakeTuple(4.3f, -4.2f, 3.1f, 1.f) };

	// Then a.x = 4.3
	ASSERT_TRUE(rlFeq(a.x, 4.3f));
	// And a.y = -4.2
	ASSERT_TRUE(rlFeq(a.y, -4.2f));
	// And a.z = 3.1
	ASSERT_TRUE(rlFeq(a.z, 3.1f));
	// And a.w = 1.0
	ASSERT_TRUE(rlFeq(a.w, 1.f));

	// And a is a point
	ASSERT_TRUE(rlIsPoint(a));
	// And a is not a vector
	ASSERT_FALSE(rlIsVector(a));
}

// Scenario: A tuple with w=0 is a vector
TEST(tuple, wZeroVector)
{
	// Given a <- tuple(4.3, -4.2, 3.1, 0.0)
	auto a{ rlMakeTuple(4.3f, -4.2f, 3.1f, 0.f) };

	// Then a.x = 4.3
	ASSERT_TRUE(rlFeq(a.x, 4.3f));
	// And a.y = -4.2
	ASSERT_TRUE(rlFeq(a.y, -4.2f));
	// And a.z = 3.1
	ASSERT_TRUE(rlFeq(a.z, 3.1f));
	// And a.w = 1.0
	ASSERT_TRUE(rlFeq(a.w, 0.f));

	// And a is not a point
	ASSERT_FALSE(rlIsPoint(a));
	// And a is a vector
	ASSERT_TRUE(rlIsVector(a));
}

// Scenario: point() creates tupes with w=1
TEST(tuple, createPoint)
{
	// Given p <- point(4, -4, 3)
	auto p{ rlMakePoint(4.f, -4.f, 3.f) };

	// Then p = tuple(4, -4, 3, 1)
	ASSERT_TRUE(rlTeq(p, rlMakeTuple(4.f, -4.f, 3.f, 1.f)));
}

// Scenario: vector() creates tuples with w=0
TEST(tuple, createVector)
{
	// Given v <- vector(4, -4, 3)
	auto v{ rlMakeVector(4.f, -4.f, 3.f) };

	// Then v = tuple(4, -4, 3, 0)
	ASSERT_TRUE(rlTeq(v, rlMakeTuple(4.f, -4.f, 3.f, 0.f)));
}

// Scenario: Adding two tuples
TEST(tuple, addTuples)
{
	// Given a1 <- tuple(3, -2, 5, 1)
	auto a1{ rlMakeTuple(3.f, -2.f, 5.f, 1.f) };
	// And a2 <- tuple(-2, 3, 1, 0)
	auto a2{ rlMakeTuple(-2.f, 3.f, 1.f, 0.f) };

	// Then a1 + a2 = tuple(1, 1, 5, 1)
	ASSERT_TRUE(rlTeq(rlTadd(a1, a2), rlMakeTuple(1.f, 1.f, 6.f, 1.f)));
}

// Scenario: Subtracting two points
TEST(tuple, subtractPoints)
{
	// Given p1 <- point(3, 2, 1)
	auto p1{ rlMakePoint(3.f, 2.f, 1.f) };
	// And p2 <- point(5, 6, 7)
	auto p2{ rlMakePoint(5.f, 6.f, 7.f) };

	// Then p1 - p2 = vector(-2, -4, -6)
	ASSERT_TRUE(rlTeq(rlTsub(p1, p2), rlMakeVector(-2.f, -4.f, -6.f)));
}

// Scenario: Subtracting a vector from a point
TEST(tuple, subtractVectorFromPoint)
{
	// Given p <- point(3, 2, 1)
	auto p{ rlMakePoint(3.f, 2.f, 1.f) };
	// And v <- vector(5, 6, 7)
	auto v{ rlMakeVector(5.f, 6.f, 7.f) };

	// Then p - v = point(-2, -4, -6)
	ASSERT_TRUE(rlTeq(rlTsub(p, v), rlMakePoint(-2.f, -4.f, -6.f)));
}

// Scenario: Subtracting two vectors
TEST(tuple, subtractVectors)
{
	// Given v1 <- vector(3, 2, 1)
	auto v1{ rlMakeVector(3.f, 2.f, 1.f) };
	// And v <- vector(5, 6, 7)
	auto v2{ rlMakeVector(5.f, 6.f, 7.f) };

	// Then v1 - v2 = vector(-2, -4, -6)
	ASSERT_TRUE(rlTeq(rlTsub(v1, v2), rlMakeVector(-2.f, -4.f, -6.f)));
}

// Scenario: Subtracting a vector from the zero vector
TEST(tuple, subtractVectorFromZero)
{
	// Given zero <- vector(0, 0, 0)
	auto zero{ rlMakeVector1(0.f) };
	// And v <- vector(1, -2, 3)
	auto v{ rlMakeVector(1.f, -2.f, 3.f) };

	// Then zero - v = vector(-1, 2, -3)
	ASSERT_TRUE(rlTeq(rlTsub(zero, v), rlMakeVector(-1.f, 2.f, -3.f)));
}

// Scenario: Negating a tuple
TEST(tuple, negateTuple)
{
	//Given a <- tuple(1, -2, 3, -4)
	auto a{ rlMakeTuple(1.f, -2.f, 3.f, -4.f) };

	// Then -a = tuple(-1, 2, -3, 4)
	ASSERT_TRUE(rlTeq(rlTneg(a), rlMakeTuple(-1.f, 2.f, -3.f, 4.f)));
}

// Scenario: Multiplying a tuple by a scalar
TEST(tuple, multiplyTupleByScalar)
{
	// Given a <- tuple(1, -2, 3, -4)
	auto a{ rlMakeTuple(1.f, -2.f, 3.f, -4.f) };

	// Then a * 3.5 = tuple(3.5, -7, 10.5, -14)
	ASSERT_TRUE(
		rlTeq(rlFTmul(3.5f, a), rlMakeTuple(3.5f, -7.f, 10.5f, -14.f)));
}

// Scenario: Multiplying a tuple by a fraction
TEST(tuple, multiplyTupleByFraction)
{
	// Given a <- tuple(1, -2, 3, -4)
	auto a{ rlMakeTuple(1.f, -2.f, 3.f, -4.f) };

	// Then a * 0.5 = tuple(0.5, -1, 1.5, -2)
	ASSERT_TRUE(
		rlTeq(rlFTmul(0.5f, a), rlMakeTuple(0.5f, -1.f, 1.5f, -2.f)));
}

// Scenario: Computing the magnitude of vector(1, 0, 0)
TEST(tuple, magnitudeOfNormalVec0)
{
	// Given v <- vector(1, 0, 0)
	auto v{ rlMakeVector(1.f, 0.f, 0.f) };

	// Then magnitude(v) = 1
	ASSERT_TRUE(rlFeq(rlTmag(v), 1.f));
}

// Scenario: Computing the magnitude of vector(0, 1, 0)
TEST(tuple, magnitudeOfNormalVec1)
{
	// Given v <- vector(0, 1, 0)
	auto v{ rlMakeVector(0.f, 1.f, 0.f) };

	// Then magnitude(v) = 1
	ASSERT_TRUE(rlFeq(rlTmag(v), 1.f));
}

// Scenario: Computing the magnitude of vector(0, 0, 1)
TEST(tuple, magnitudeOfNormalVec2)
{
	// Given v <- vector(0, 0, 1)
	auto v{ rlMakeVector(0.f, 0.f, 1.f) };

	// Then magnitude(v) = 1
	ASSERT_TRUE(rlFeq(rlTmag(v), 1.f));
}

// Scenario: Computing the magnitude of vector(1, 2, 3)
TEST(tuple, magnitudeOfVec0)
{
	// Given v <- vector(1, 2, 3)
	auto v{ rlMakeVector(1.f, 2.f, 3.f) };

	// Then magnitude(v) = sqrt(14)
	ASSERT_TRUE(rlFeq(rlTmag(v), sqrtf(14.f)));
}

// Scenario: Computing the magnitude of vector(-1, -2, -3)
TEST(tuple, magnitudeOfVec1)
{
	// Given v <- vector(-1, -2, -3)
	auto v{ rlMakeVector(-1.f, -2.f, -3.f) };

	// Then magnitude(v) = sqrt(14)
	ASSERT_TRUE(rlFeq(rlTmag(v), sqrtf(14.f)));
}

// Scenario: Normalizing vector(4, 0, 0) gives (1, 0, 0)
TEST(tuple, normalizeVec0)
{
	// Given v <- vector(4, 0, 0)
	auto v{ rlMakeVector(4.f, 0.f, 0.f) };

	// Then normalize(v) = vector(1, 0, 0)
	ASSERT_TRUE(rlTeq(rlTnrm(v), rlMakeVector(1.f, 0.f, 0.f)));
}

// Scenario: Normalizing vector(1, 2, 3)
TEST(tuple, normalizeVec1)
{
	// Given v <- vector(1, 2, 3)
	auto v{ rlMakeVector(1.f, 2.f, 3.f) };

	auto mag{ sqrtf(14.f) };

	// Then normalize(v) = vector(1/sqrt(14), 2/sqrt(14), 3/sqrt(14))
	ASSERT_TRUE(rlTeq(rlTnrm(v),
			  rlMakeVector(1.f / mag, 2.f / mag, 3.f / mag)));
}

// Scenario: The magnitude of a normalized vector
TEST(tuple, magnitudeOfNormalVec)
{
	// Given v <- vector(1, 2, 3)
	auto v{ rlMakeVector(1.f, 2.f, 3.f) };

	// When norm <- normalize(v)
	auto norm{ rlTnrm(v) };

	// Then magnitude(norm) = 1
	ASSERT_TRUE(rlFeq(rlTmag(norm), 1.f));
}
