# hash_test
Test the speed of Blake3 vs SHA hashes. It hashes a file in the current working directory named test. <BR>
When I test this, I can't tell if the Blake3 hash is muti-threaded. <BR>
To greate a 4GB test file, please run the following command <BR>
```sh
dd if=/dev/urandom of=test bs=64M count=128 iflag=fullblock
```