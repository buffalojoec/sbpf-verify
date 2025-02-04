# SBPF Verify

In order to verify a program ELF on-chain, we must contend with (at least) two
constraints: heap size and compute unit meter.

The `Executable` struct from `solana-sbpf` imposes a heap allocation when
initializing:

https://github.com/anza-xyz/sbpf/blob/613bf9bc6de094bc8995423c78d4b850f922dc19/src/elf.rs#L613-L614

A workaround could be obtained by reimplementing the parser and avoiding the
ownership, but something must be done for relocations.

Furthermore, even with a workaround for heap, we'll need to sort out breaking
up the job to avoid the CU limit. Without a syscall for "remaining CUs", this
will have to be a bit expensive for the caller.

* One job does the initial parsing of ELF sections and stores offsets in some
  account. Can also check for valid header values, etc.
* Subsequent jobs seek to verify the ELF chunk by chunk.
    * Incrementally stores the program counter (PC) in some account.
    * Once finished, stores the verification status in the account.

This is a bit annoying, since you might have to send a bunch of separate
instructions, or somehow use inputs, to continually invoke the program until it
finishes.

One other piece of this system is captured in the program's processor under the
[`program_runtime_environment`](./src/processor.rs#L12) module. On-chain
programs don't really have a well-baked solution for keeping up with cluster
software versions and their respective feature sets. We may need a
sysvar/syscall for this.
