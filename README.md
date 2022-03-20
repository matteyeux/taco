# taco

[![Crates.io](https://img.shields.io/crates/v/taco)](https://crates.io/crates/taco)

A tool to download and decrypt 64 bits iOS firmware images


### Usage
```
λ ~ » taco
taco 0.1.0
Tool to automatically download and decrypt 64 bits iOS firmware images.

USAGE:
    taco <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    decrypt     Decrypt firmware image
    download    Download firmware image
    help        Print this message or the help of the given subcommand(s)
    info        info about device
```

### Run

Make sure to have [img4](https://github.com/xerub/img4) available somewhere in your `$PATH`.

Simple usage:
```
λ ~ » taco decrypt iPad7,3 15.1 iBoot.j207.RELEASE.im4p  
[i] Downloading iBoot.j207.RELEASE.im4p
[i] Grabbing keys for iPad7,3/19B74
[x] IV  : f70605bdd8202c2f08407b3f791dc7a2
[x] Key : 82c61a0c1bde18583ec4476cc9eda52f5f7fce46c76f73f5c44e2d423402846d
[i] Decrypting iBoot.j207.RELEASE.im4p to iBoot.j207.RELEASE.bin
```

Specify key instead of grabbing it from the wiki:
```
λ ~ » taco decrypt iPad7,3 15.1 iBoot.j207.RELEASE.im4p -k f70605bdd8202c2f08407b3f791dc7a282c61a0c1bde18583ec4476cc9eda52f5f7fce46c76f73f5c44e2d423402846d
[i] Downloading iBoot.j207.RELEASE.im4p
[x] IV  : f70605bdd8202c2f08407b3f791dc7a2
[x] Key : 82c61a0c1bde18583ec4476cc9eda52f5f7fce46c76f73f5c44e2d423402846d
[i] Decrypting iBoot.j207.RELEASE.im4p to iBoot.j207.RELEASE.bin
```


Use already downloaded file:
```
λ ~ » taco decrypt iPad7,3 15.1 iBoot.j207.RELEASE.im4p -l
[i] Grabbing keys for iPad7,3/19B74
[x] IV  : f70605bdd8202c2f08407b3f791dc7a2
[x] Key : 82c61a0c1bde18583ec4476cc9eda52f5f7fce46c76f73f5c44e2d423402846d
[i] Decrypting iBoot.j207.RELEASE.im4p to iBoot.j207.RELEASE.bin
```

Download an decrypt iBoot from iOS 15.0 beta beta 6 for iPhone 12 Pro Max:
```
λ ~ » taco decrypt iPhone13,4 19A5325f iBoot.d54p.RELEASE.im4p -b
   Compiling taco v0.1.1 (/home/mathieu/dev/taco)
    Finished dev [unoptimized + debuginfo] target(s) in 3.85s
     Running `target/debug/taco decrypt iPhone13,4 19A5325f iBoot.d54p.RELEASE.im4p -b`
[i] Downloading iBoot.d54p.RELEASE.im4p
[i] Grabbing keys for iPhone13,4/19A5325f
[x] IV  : 12b3b21327cdde96ead01b3ee1605ee7
[x] Key : a7b73b08cab0e699b94eeb057ed7800241e8db7d1618fa45de68ef8448746444
[i] Decrypting iBoot.d54p.RELEASE.im4p to iBoot.d54p.RELEASE.bin
```

### TODO 
- [X] Specify keys without grabbing them from the wiki
- [X] Use file locally
- [X] Support for beta iOS versions
- [ ] [foreman](https://github.com/GuardianFirewall/foreman) Support
- [ ] Decode and decrypt payload without [img4](https://github.com/xerub/img4)

### Credits

- Marco Grassi : [partialzip](https://github.com/marcograss/partialzip)

