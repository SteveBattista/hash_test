# hash_test
Test the speed of Blake3 vs SHA hashes. It hashes a file in the current working directory named test. <BR>
To create a 8GB test file, please run the following command:<BR>
```sh
dd if=/dev/urandom of=test bs=64M count=128 iflag=fullblock
```
Please note that the conversion funtion uses 1,000 rather than 1,024 as delimter betwen file sizes.

Running on a Dell6400 with:<br>
Ubuntu 20.10 with only GUI and terminal running.<br>
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
blake3 value is "7C6E8A0E9DF306FC83E8ED32FFEDFCFA0C78C6F32D197E202F6D0AB9BA41E368"
blake3 took 3069 milliseconds. Speed is 2.8 GBs
File is being hashed with memmap.
128 value is "016AE178316B60B96AF010247CE4FFA5364F0EF5"
128 took 43258 milliseconds. Speed is 198.57 MBs
File is being hashed with memmap.
256 value is "20143DD213AFB4ABDC834FBA95A8DE46A04033BD3F9A3E34C62B78E6FD4AB9EB"
256 took 30190 milliseconds. Speed is 284.53 MBs
File is being hashed with memmap.
384 value is "D4815F4F74833497D9AF9D2426C02DB5D94FC8BBA737636A32754FEDCFDCCF8867B439D03EA693E68BCBE514EA99AD77"
384 took 21242 milliseconds. Speed is 404.38 MBs
File is being hashed with memmap.
512 value is "DE126272F1D57E824BB7B24579163A561B4F691D01DBF654D1B02177DB08E15F6DB8AB0BCFD31223490E96A1427F2175C8797B341D623F5E7AB1C9AB96797BD5"
512 took 21250 milliseconds. Speed is 404.23 MBs
File is being hashed with memmap.
512_256 value is "C4A12BA39AAFC38682AC6CE931489116233C8AF85359DACF97BE6B551BB3055A"
512_256 took 21236 milliseconds. Speed is 404.5 MBs
```
