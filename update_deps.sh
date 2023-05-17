curl 'https://raw.githubusercontent.com/torvalds/linux/master/.clang-format' > .clang-format

echo '# From https://github.com/torvalds/linux\n' | cat - .clang-format > /tmp/clang-format.tmp
mv /tmp/clang-format.tmp .clang-format
