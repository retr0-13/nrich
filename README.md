# nrich

A command-line tool to quickly analyze all IPs in a file and see which ones have open ports/ vulnerabilities. Can also be fed data from stdin to be used in a data pipeline.

<p>
<a href="https://opensource.org/licenses/GPL-3.0"><img src="https://img.shields.io/badge/license-GPL3-_red.svg"></a>
<a href="https://rust-reportcard.xuri.me/report/gitlab.com/shodan-public/nrich"><img src="https://rust-reportcard.xuri.me/badge/gitlab.com/shodan-public/nrich"></a>
<a href="https://twitter.com/shodanhq"><img src="https://img.shields.io/twitter/follow/shodanhq.svg?logo=twitter"></a>
</p>

## Installation

Grab the [latest release](https://gitlab.com/shodan-public/nrich/-/releases) for your operating system. For example, to install the ``nrich`` command in Ubuntu:

```shell
$ wget https://gitlab.com/api/v4/projects/33695681/packages/generic/nrich/latest/nrich_latest_amd64.deb
$ sudo dpkg -i nrich_latest_amd64.deb
```

To confirm that it's working you can pipe an IP to the command:

```shell
echo 1.1.1.1 | nrich -
```

## Usage

The ``nrich`` command only requires a single argument: the filename that contains the IPs:

```shell
$ nrich --help
nrich 0.1.0
Add network information to IPs

USAGE:
    nrich [OPTIONS] <filename>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <output>    Output format (shell or json) [default: shell]

ARGS:
    <filename>    File containing an IP per line. Non-IPs are ignored
```

Here is a short video that shows how to install ``nrich`` and then run it against a list of emerging threats:

[![asciicast](https://asciinema.org/a/468923.svg)](https://asciinema.org/a/468923)
