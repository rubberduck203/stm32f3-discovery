# VS Code Configuration

Example configuration for debugging programs in-editor with VS Code.  

## Required Extensions

If you have the `code` command in your path, you can run the following commands to install the necessary extensions.

```sh
code --install-extension rust-lang.rust
code --install-extension marus25.cortex-debug
```

Otherwise, you can use the Extensions view to search for and install them, or go directly to their marketplace pages and click the "Install" button.

- [Rust Language Server (RLS)](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
- [Cortex-Debug](https://marketplace.visualstudio.com/items?itemName=marus25.cortex-debug)

## Use

_Note: When you open the project in the editor, you must open an `*.rs` file to trigger the Rust Language Server.
Failure to do so will cause a failure to find the `build` task._

The `Debug (OpenOCD)`: Starts a debug session for a `STM32F3DISCOVERY` board.
It will use the default `.cargo/config` configuration to build the executable, upload it to the device, and start a debug session.

`ITM` output, if used, will be written to the Output view `SWO: ITM [port: 0, type: console]` output.

### Git

Files in the `.vscode/` directory are `.gitignore`d by default because many files that may end up in the `.vscode/` directory should not be committed and shared.  
However, a number of files are explicitly tracked, because they define complex debug configurations and should be shared with anyone cloning your project.  

### SVD File

The SVD file is a standard way of describing all registers and peripherals of an ARM Cortex-M mCU.  
Cortex-Debug needs this file to display the current register values for the peripherals on the device.  

For licensing reasons, we're unable to include the SVD file in the quickstart repository, but it can be downloaded from the [ST's Website][stm32f3].

Download the [stm32f3 SVD pack][stm32f3-svd], and copy the `STM32F303.svd` file into `~/.svd/`.  
This line of the config tells the Cortex-Debug plug in where to find the file.

```json
"svdFile": "${env:HOME}/.svd/STM32F303.svd",
```

Personally, I like keeping them in my home directory so I don't have to keep multiple copies of the SVD on disk, but you could also keep the file under `.vscode/` if you have a private project where there are no licensing concerns around the SVD file.

```json
"svdFile": "${workspaceRoot}/.vscode/STM32F303.svd",
```

### CPU Frequency

If your device is running at a frequency other than the default 8MHz, you'll need to modify this line of `launch.json` for the `ITM` output to work correctly.

```json
"cpuFrequency": 8000000,
```

<!-- references -->

[cortex-debug]: https://github.com/Marus/cortex-debug
[stm32f3]: https://www.st.com/content/st_com/en/products/microcontrollers-microprocessors/stm32-32-bit-arm-cortex-mcus/stm32-mainstream-mcus/stm32f3-series.html#resource
[stm32f3-svd]: https://www.st.com/resource/en/svd/stm32f3_svd.zip
[openocd-config]: http://openocd.org/doc/html/Config-File-Guidelines.html
[openocd-repo]: https://sourceforge.net/p/openocd/code/ci/master/tree/tcl/
