curl 'https://raw.githubusercontent.com/torvalds/linux/master/.clang-format' > .clang-format

attrib_string=$'# From https://github.com/torvalds/linux\n'

echo "$attrib_string" | cat - .clang-format > /tmp/clang-format.tmp
mv /tmp/clang-format.tmp .clang-format
