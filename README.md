# Sherlock Pomodoro

This is a very lightweight pomodoro timer built in Rust for usage with the
[Sherlock Application Launcher](https://github.com/Skxxtz/sherlock). It listens
for socket messages to communicate with the Sherlock main process. Sherlock can then display the remaining time within the application.

## Installation

### <ins>From Source</ins>

To build Sherlock Pomodoro from source, follow these steps.<br>
Make sure you have the necessary dependencies installed:

- `rust` - [How to install rust](https://www.rust-lang.org/tools/install)
- `git` - [How to install git](https://github.com/git-guides/install-git)

1. **Clone the repository**:

    ```bash
    git clone https://github.com/skxxtz/sherlock-pomodoro.git
    cd sherlock-pomodoro
    ```

2. **Build the project**:

    ```bash
    cargo build --release
    ```

3. **Install the binary**:

    After the build completes, install the binary to your system:

    ```bash
    sudo cp target/release/sherlock-pomodoro /usr/local/bin/
    ```

### <ins>From Binary</ins>

Make sure you have the following dependency installed:

- `tar` - [Tar](https://www.gnu.org/software/tar/)

1. **Download the archive containing the latest release**:

    The archive can be found [here](https://github.com/Skxxtz/sherlock-pomodoro/releases/latest).

2. **Extract the files from the archive**:

    ```bash
    cd ~/Downloads/
    tar -xzf sherlock-pomodoro*.tar.gz
    ```
    You can use tab-completion or run `ls` to verify the name of the archive.

3. **Install the binary**:

    Now move the binary to a location on your `$PATH` environment variable:

    ```bash
    sudo mv sherlock-pomodoro /usr/local/bin/
    ```

    Optionally also move the LICENSE file or delete it:

    ```bash
    sudo mkdir -p /usr/share/doc/sherlock-pomodoro
    sudo mv LICENSE /usr/share/doc/sherlock-pomodoro/

    # or, to remove it:
    rm LICENSE
    ```

### <ins>Build Debian Package</ins>

To build a `.deb` package directly from the source, follow these steps:<br>
Make sure you have the following dependencies installed:

- `rust` - [How to install rust](https://www.rust-lang.org/tools/install)
- `git` - [How to install git](https://github.com/git-guides/install-git)

1. **Install the `cargo-deb` tool**:

    First, you need to install the `cargo-deb` tool, which simplifies packaging Rust projects as Debian packages:

    ```bash
    cargo install cargo-deb
    ```

2. **Build the Debian package**:

    After installing `cargo-deb`, run the following command to build the `.deb` package:

    ```bash
    cargo deb
    ```

    This will create a `.deb` package in the `target/debian` directory.

3. **Install the generated `.deb` package**:

    Once the package is built, you can install it using:

    ```bash
    sudo dpkg -i target/debian/sherlock-pomodoro_*.deb
    ```

    You can use tab-completion or `ls target/debian/` to confirm the file name.

    (Make sure to replace the filename if the version number is different.)

