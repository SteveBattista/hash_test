# hash_test
Test the speed of Blake3 vs SHA hashes. It hashes a file in the current working directory named test. <BR>
To create a 8GB test file, please run the following command:<BR>
```sh
dd if=/dev/urandom of=test bs=64M count=128 iflag=fullblock
```
Please note that the conversion function uses 1,000 rather than 1,024 as delimiter between file size units.

Running on a Dell6400 with:<br>
Ubuntu 21.10 on a server.<br>
Using memap<br>
Intel(R) Core(TM) i7-2620M CPU @ 2.70GHz (2 cores 4 threads)<br>
16MB of 1333 MT/s Ram<br>
Spinning hard disk: WDC WD2500BEKT-00PVMT0 <br>
<br>

```
Input file named ./test is 8.59 GB in size.
Ensuring file is cached
File is being hashed with memmap.
File is cached.
File is being hashed with memmap.
blake3 value is "C30B1792691F469BB215F23B182ED8157BD96398536D2472D25AA131160ADDFA"
blake3 took 3312 milliseconds. Speed is 2.59 GBs
File is being hashed with memmap.
128 value is "93927B51EF36EA297F7199889DC639FC0953C7BB"
128 took 28788 milliseconds. Speed is 298.38 MBs
File is being hashed with memmap.
256 value is "8D78BB56F7C97D579FC94006F0270FDEFCC97F378FE50F24C38F94C8F1322DE4"
256 took 29841 milliseconds. Speed is 287.86 MBs
File is being hashed with memmap.
384 value is "7818C1E2EB9BB43B9A93C0037397CE201E605DA5BFAC730B9305CE18341AD663DC2B01559F224F9FB9660A2E6A9883B2"
384 took 20936 milliseconds. Speed is 410.29 MBs
File is being hashed with memmap.
512 value is "F5AA8FE38B3A8EF6F0D14363DB7AE2DEBC003AA0559EC935D433E8D584CD88BD302ADAEE1851D4FE5545AD7EBE243D584EDD39A09A0CB3B302BB87160A5B8D1A"
512 took 20968 milliseconds. Speed is 409.67 MBs
File is being hashed with memmap.
512_256 value is "0E25EA149D45A77D01B0A4CFB0D4F514BBF2C5F9EE9E49F1E163B8DB7C774894"
512_256 took 20949 milliseconds. Speed is 410.04 MBs
File is being hashed with memmap.
MD5 value is "2C44BC4016FEE1191D42269382823214"
MD5 took 23846 milliseconds. Speed is 360.23 MBs


```
