# hash_test
Test the speed of Blake3 vs SHA hashes. It hashes a file in the current working directory named test. <BR>
To create a 4GB test file, please run the following command <BR>
```sh
dd if=/dev/urandom of=test bs=64M count=128 iflag=fullblock
```
