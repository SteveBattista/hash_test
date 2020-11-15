# hash_test
Test the speed of Blake3 vs SHA hashes. <BR>
When I test this, I can't tell if the Blake3 hash is not muti-threaded. <BR>
To greate a 4GB test file run the following command <BR>
```sh
dd if=/dev/urandom of=test bs=64M count=128 iflag=fullblock
```