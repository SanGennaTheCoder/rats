# rats
an easy to customize CLI tool to display random text and ASCII art

# installation
## Linux
here are the steps to install it on linux systems:
1. chmod +x [path/of/binary]
2. sudo mv [path/of/binary] /usr/local/bin/rats

Explanation:
1. makes the binary executable
2. moves the executable to PATH so you dont need to source it to make it work

To verify the installation:
1. which rats

if it outputs:
/usr/local/bin/rats

then the installation is correct!
else there's an error

another way to install is just to source the binary you installed in the current terminal session
and run it with the --install flag

1. .[path/of/binary]
2. rats --install

if it prints:
"successfully installed to [path]" -> the binary was successfully copied to ~/.local/bin and is ready to use.

"rats is already installed at [path]" -> the binary already exists in ~/.local/bin, no changes were made.

"failed to copy binary" -> the program tried to copy itself to ~/.local/bin but encountered an error.  
   This could happen if you lack write permissions or the disk is full.