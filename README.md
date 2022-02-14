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
blake3 value is "B9568128E1CB4B98870972414A6EDFE9000F5CAB8CC884D396651A119EFB9EC7"
blake3 took 2988 milliseconds. Speed is 2.87 GBs
File is being hashed with memmap.
128 value is "A41BF1F5DF51EE5AADA4BEBAB7B374C7997C0E54"
128 took 42363 milliseconds. Speed is 202.77 MBs
File is being hashed with memmap.
256 value is "39D384DF218C2AB2A86F2B7D15C4437F85FBCD05867DCF76C11AEC139D941605"
256 took 29830 milliseconds. Speed is 287.96 MBs
File is being hashed with memmap.
384 value is "F1DDD8A0E9761CA230ACE5CDBA2FA2326776A513BA2BCAD50CDD9F3DCF1F7DDD12A50E2A5AA42A432E54296F39331CB0"
384 took 20941 milliseconds. Speed is 410.2 MBs
File is being hashed with memmap.
512 value is "1B6804BF60F522BBF20F8CA836081770ED626CA071B3EE3E1423946257E3FBD09EF53DFAD9F91C3B47003434698D93118523F99F30B3C9A9F25ACBDA13738D28"
512 took 20961 milliseconds. Speed is 409.81 MBs
File is being hashed with memmap.
512_256 value is "0DF96FD4586942E93811376BE0A618B4888BBE05D80DEE3111B4668416843DFF"
512_256 took 20943 milliseconds. Speed is 410.16 MBs

```
