<p align="center"><img src="https://github.com/pwnwriter/kanha/blob/logos/shree.svg" width="250px" height="100px" >
<h4 align="center"><strong><code>Kanha</code></strong> - A web-app pentesting suite written in rust 🦀</h4> </h6>

<h6 align="center">
  <a href="https://github.com/pwnwriter/kanha#-installation"><code>Installation</code></a>
  ⦾
  <a href="https://github.com/pwnwriter/kanha#-Subcommands"><code>Subcommands</code></a>
  ⦾
  <a href="https://github.com/pwnwriter/kanha#-contributing"><code>Contribute</code></a>
 </h6>
<p align="center">
<a href="https://crates.io/crates/kanha/"><img src="https://img.shields.io/crates/v/kanha?style=flat&amp;labelColor=56534b&amp;color=c1c1b6&amp;logo=Rust&amp;logoColor=white" alt="Crate Release"></a>
<a href="https://github.com/pwnwriter/kanha/issues"><img src="https://img.shields.io/github/issues/pwnwriter/kanha.svg?style=flat-square&label=Issues&color=d77982"></a>
<a href="https://github.com/pwnwriter/pwnwriter/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-MIT-white.svg" alt="MIT LICENSE"></a>
<a href="https://ko-fi.com/pwnwriter"><img src="https://img.shields.io/badge/support-pwnwriter%20-pink?logo=kofi&logoColor=white" alt="Ko-fi"></a>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/aqua.png)

<img src="https://github.com/pwnwriter/kanha/blob/logos/kanha-help.png" alt="img" align="right" width="50%"></p>
    
[**`Kanha`**](/) is a tool that can help you perform, a variety of attacks based on the target domain . With just `kanha` you can do, [***`Fuzzing`***](https://en.wikipedia.org/wiki/Fuzzing), [***`Reverse dns lookup`***](https://en.wikipedia.org/wiki/Reverse_DNS_lookup),
[***`common http response`***](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes), [***`subdomain takeover detection`***](https://en.wikipedia.org/wiki/Domain_hijacking) and many [**`more`**](/src/commands). 

The project is inspird by [`mini.nvim`](https://github.com/echasnovski/mini.nvim), basically helping you to be productive with less numbers of *tools(plugins)* installed on your system and be unobtrusive and function as a standalone **`single binary`** out of the box.

Built from the ground up with performance, ease of use, and portability in mind in your favourite programming lang [**`rust`**](https://www.rust-lang.org/) 💝

## 🧠 Philosophy

- **KISS** - Keep things simple and stupid.
- **Ease** - Write code that can be used elsewhere as well.
- **Efficiency** - Optimize for performance without sacrificing readability.

## 🐱 Installation 
    
  <details> <summary><code>🪄 Binary </code></summary>
    &nbsp;

  - You can directly download the [**binary**](https://github.com/pwnwriter/kanha/releases) of your arch and run it.
  
  </details>
  <details> <summary><code>🌼 Source </code></summary>
  &nbsp;
 
  ```bash
  git clone --depth=1 https://github.com/pwnwriter/kanha --branch=main
  cd kanha
  cargo build --release 
  ```
  Then go to `release` dir and `./kanha` or move the `binary` to your any `$PATH` for instant access from anywhere.
</details>

<details> <summary><code>🎠 Cargo </code></summary>

- Using [crates.io](https://crates.io/crates/kanha)
  ```bash
  cargo install kanha
  ```
- Using [binstall](https://github.com/cargo-bins/cargo-binstall)
  ```bash
  cargo binstall kanha
  ```

  > **Note** ⚠️
  > This requires a working setup of rust/cargo & binstall.
</details>

<details> <summary><code>🚩 METIS Linux </code></summary>
&nbsp;
  
  ```bash
  sudo/doas pacman -Syyy kanha
  ```

</details>

## 🌈 Subcommands
- ➊ `Status` :- Just return the HTTP response code of URLs

  <details>
  <summary>👻 Help</summary>
  &nbsp;

  ```bash
  $ kanha status -h
  
  Just return the HTTP response code of URLs

  Usage: kanha status [OPTIONS]

    Options:
    -f, --filename <FILENAME>  A url or a file containing multiple urls
        --stdin                Reads input from the standard in
    -t, --tasks <TASKS>        Define the maximum concurrent tasks [default: 10]
    -h, --help                 Print help
    -V, --version              Print version
  ```

  <details>
  <summary>🦊 Screenshots </summary>
      &nbsp;
    
  ![status](https://github.com/pwnwriter/kanha/assets/90331517/93f7656f-563c-4c92-a118-500b1fabae9e)
  ![status-stdin](https://github.com/pwnwriter/kanha/assets/90331517/5ac0d6c6-497a-4a8d-a1a2-d3326010d7a8)  

  </details>

</details>

- ➋  `fuzz` :- Fuzz URLs and return the response codes
    
  <details>
  <summary>👻 Help</summary>
  &nbsp;
    
  ```bash
  $ kanha fuzz -h
  Fuzz URLs and return the response codes

  Usage: kanha fuzz [OPTIONS] --wordlist <WORDLIST> --url <URL>

  Options:
    -w, --wordlist <WORDLIST>  A file containing a list of possible wordlists
    -u, --url <URL>            Provide a url to fuzz
    -t, --tasks <TASKS>        Define the maximum concurrent tasks [default: 10]
    -h, --help                 Print help
    -V, --version              Print version
  ```
    <details>
  <summary>🦊 Screenshots </summary>
      &nbsp;
      
  ![fuzz](https://github.com/pwnwriter/kanha/assets/90331517/171d5fb8-b4b1-480c-9331-4204fa44944f)
  </details>
  
  </details>

- ➌ `rdns` :- Reverse dns lookup
    <details>
  <summary>👻 Help</summary>  
  &nbsp;
      
  ```bash
  
  $ kanha rdns  -h
    Reverse dns lookup

    Usage: kanha rdns [OPTIONS] --filename <FILENAME>

    Options:
      -f, --filename <FILENAME>  a file containing a list of possible wordlists
          --stdin                Reads input from the standard in
      -h, --help                 Print help
      -V, --version              Print version
  ```
    <details>
  <summary>🦊 Screenshots </summary>
      &nbsp;
      
  ![rdns](https://github.com/pwnwriter/kanha/assets/90331517/44f2f7f1-9f47-4794-87e9-1366b4a3e443)
  ![rdns-stdin](https://github.com/pwnwriter/kanha/assets/90331517/9ad5e5b6-711e-4396-a46f-5c190000e185)

  </details>
</details>

- ➍ `Takeover` :- Check possible subdomain takeover
    <details>
  <summary>👻 Help</summary>  
  &nbsp;
      
  ```bash
  Check possible subdomain takeover

  Usage: kanha takeover [OPTIONS] --json-file <JSON_FILE>

  Options:
    -j, --json-file <JSON_FILE>  A json file containing signature values of different services
    -f, --filename <FILENAME>    A file containing a list of urls
        --stdin                  Reads input from the standard in
    -h, --help                   Print help
    -V, --version                Print version
  ```
    <details>
  <summary>🦊 Screenshots </summary>
      &nbsp;
      
  ![takeover](https://github.com/pwnwriter/kanha/assets/90331517/25d499b0-8e66-4cc5-a414-887deb10124f)
  ![takeover-stdin](https://github.com/pwnwriter/kanha/assets/90331517/1b956c9d-2d37-4656-97ee-2aca2199750b)
  </details>
</details>

<!-- ➎ ➏ ➐ ➑ ➒ -->

## 👐 Contributing
  - 🪶 Recommend a new features
  - ⭐ Give the project a star
  - 🐎 Add new [subcommand](/src/commands).
  - 🧑‍🚒 Fix docx // improve code quality

## Also see 👀
- [`haylxon`](https://github.com/pwnwriter/haylxon) :- Blazingly fast tool to grab screenshots of your domain list right from terminal written in rust 🦀
- [`httpx`](https://github.com/projectdiscovery/httpx) :- httpx is a fast and multi-purpose HTTP toolkit.
- [`ffuf`](https://github.com/ffuf/ffuf) :- Fast web fuzzer written in Go

## License 🔐
 As always, this project is also licensed under the [**`MIT LICENSE`**](/LICENSE) 
 &nbsp;
 
<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
<p align="center">Copyright &copy; 2023<a href="https://pwnwriter.xyz" target="_blank"> pwnwriter xyz </a> ☘️ </p> 
  

