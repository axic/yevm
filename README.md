# yevm

"Why EVM?" is an EVM compiler infrastructure. The first target output is [ewasm](https://github.com/ewasm).

Internally it works as compiling EVM input to [Yul](https://solidity.readthedocs.io/en/develop/yul.html) and
then utilising the Yul compiler infrastructure to generate optimised output.

Compiling EVM input to Yul means it will result in an output similar to what Solidity is expected to output:
it will contain 256-bit operations. However, WebAssembly can only deal with 64-bit operations. Yevm will contain
helpers to implement 256-bit operations using 64-bit operations only. These helpers are going to be used by Solidity
in order to target ewasm.

As a result, the work on yevm will also benefit speeding up the process of having native support for ewasm in Solidity.

The main challenge is "functionalising" input with jumps to unknown locations. This will be suboptimal, however
by detecting known patterns Solidity outputs, it will be possible to determine function declarations and function
calls to generate more optimal code.

Furthermore, if *EVM1.5* would come into life, it would make the compilation from EVM1.5 to Yul much more simpler
and efficient.
