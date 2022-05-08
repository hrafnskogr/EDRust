# EDRust
Userland EDR PoC - For educational purposes

# Quick details
- ProcessStart : simple C# project to monitor the creation of processes. It outputs names and PIDs of detected started process
- Overlord : Executable responsible for processing the input from 'ProcessStart', and injecting the library into the desired processes
- Vigilante : Dll responsible for hooking specific functions (userland) and apply 'protection'
