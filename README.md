# nuc-build-led

This is a simple daemon-type program that can be used alongside the Intel NUC LED kernel
module to display status associated with a build system. The kernel module can be found
here: [https://github.com/milesp20/intel_nuc_led/].

It creates a unix-domain socket at `/var/run/nuc-led/control`. Sending JSON to this 
socket will update the internal state of the daemon, and it will set the LED
appropriately. The state is simply a JSON object, which is merged with whatever
object is sent to the socket.

# Building

`cargo build --release`

# Using

First, make sure you're using a NUC and have the kernel module loaded. Now, as a user
with permission for the `/proc/acpi/nuc_led` control file, you can execute the
following commands to run the daemon:

```
mkdir /var/run/nuc-led
cargo run --release
```

If the daemon starts successfully, it will cycle the LED through all available colours 
(this not only indicates it's running but is helpful if invoked on reboot to clear 
the LED state, which is remembered).

The `socat` tool can be used to update the internal state. For example, this can be 
used to indicate that a project called 'test-project' is building:

```
echo '{ "test-project": { "building": true } }' | socat - UNIX-CONNECT:/var/run/nuc-led/control
```

Here are some other possible states that can be set:

```
echo '{ "test-project": { "testing": true } }' | socat - UNIX-CONNECT:/var/run/nuc-led/control
echo '{ "test-project": { "uploading": true } }' | socat - UNIX-CONNECT:/var/run/nuc-led/control
echo '{ "test-project": { "color": "blue" } }' | socat - UNIX-CONNECT:/var/run/nuc-led/control
echo '{ "test-project": { "error": true } }' | socat - UNIX-CONNECT:/var/run/nuc-led/control
echo '{ "test-project": { } }' | socat - UNIX-CONNECT:/var/run/nuc-led/control
```

This is quite convenient when the system is being used with a build system such as
Jenkins as it provides a quick visual cue as to the system's state.
