# Simple Rust TFTP Implementation

Currently just a basic implementation where a client can request files from the server. Client and server are both in the same executable.

## Current Issues
- No error mesages currently implemented
- RRQ mode field is currently ignored
- No WRQ implemented 🤷‍♂️

## Future Plans
- Implement extended options such as adjustable block size and window size
- Implement async handling of requests on the server
- Figure out more performant sending of data like the Win32 TransmitFile using Kernel APCs.
- Improve the TUI
- Add logging to file instead of spamming the terminal
