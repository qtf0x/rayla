#!/bin/bash

build_executable="OFF"
build_lib="OFF"
build_tests="OFF"
build_all=false
build_release=false
build_debug=false
build_sanitized=false
defaults=true

while getopts 'aeltrds' OPTION; do
  case "$OPTION" in
    a)
      build_all=true
      ;;
    e)
      build_executable="ON"
      ;;
    l)
      build_lib="ON"
      ;;
    t)
      build_tests="ON"
      build_lib="ON"
      ;;
    r)
      build_release=true
      defaults=false
      ;;
    d)
      build_debug=true
      defaults=false
      ;;
    s)
      build_sanitized=true
      defaults=false
      ;;
    ?)
      echo "script usage: $(basename \$0) [-a] [-l] [-t] [-r] [-d] [-s]" >&2
      exit 1
      ;;
  esac
done

if [ "$OPTIND" = 1 ] ; then

  echo "No options given - I don't know what to do!"
  echo "Please tell me what to build..."

  exit 1

fi

rm -rf build

num_proc=$(($(nproc --all) + 1))

if [ "$build_all" = true ] ; then

  cmake -B build/release -DCMAKE_BUILD_TYPE=RELEASE -DBUILD_EXECUTABLE=ON -DBUILD_LIB=ON -DBUILD_TESTS=ON
  cmake -B build/debug -DCMAKE_BUILD_TYPE=DEBUG -DBUILD_EXECUTABLE=ON -DBUILD_LIB=ON -DBUILD_TESTS=ON
  cmake -B build/sanitized -DCMAKE_BUILD_TYPE=SANITIZED -DBUILD_EXECUTABLE=ON -DBUILD_LIB=ON -DBUILD_TESTS=ON

  cmake --build build/release --parallel"$num_proc"
  cmake --build build/debug --parallel"$num_proc"
  cmake --build build/sanitized --parallel"$num_proc"

  exit 0

fi

if [ "$build_release" = true ] ; then

  cmake -B build/release -DCMAKE_BUILD_TYPE=RELEASE -DBUILD_EXECUTABLE="$build_executable" -DBUILD_LIB="$build_lib" -DBUILD_TESTS="$build_tests"

  cmake --build build/release --parallel"$num_proc"

fi

if [ "$build_debug" = true ] ; then

  cmake -B build/debug -DCMAKE_BUILD_TYPE=DEBUG -DBUILD_EXECUTABLE="$build_executable" -DBUILD_LIB="$build_lib" -DBUILD_TESTS="$build_tests"

  cmake --build build/debug --parallel"$num_proc"

fi

if [ "$build_sanitized" = true ] ; then

  cmake -B build/sanitized -DCMAKE_BUILD_TYPE=SANITIZED -DBUILD_EXECUTABLE="$build_executable" -DBUILD_LIB="$build_lib" -DBUILD_TESTS="$build_tests"

  cmake --build build/sanitized --parallel"$num_proc"

fi

if [ "$defaults" = true ] ; then

  cmake -B build/release -DCMAKE_BUILD_TYPE=RELEASE -DBUILD_EXECUTABLE="$build_executable" -DBUILD_LIB="$build_lib" -DBUILD_TESTS="$build_tests"

  cmake --build build/release --parallel"$num_proc"

fi
