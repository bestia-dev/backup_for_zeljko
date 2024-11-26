[//]: # (auto_md_to_doc_comments segment start A)

# backup_for_zeljko

[//]: # (auto_cargo_toml_to_md start)

**simple gui backup program tailored for my friend Zeljko**  
***version: 2024.1126.1404 date: 2024-11-26 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/backup_for_zeljko)***

 ![tutorial](https://img.shields.io/badge/tutorial-orange)
 ![egui](https://img.shields.io/badge/egui-orange)
 ![rust](https://img.shields.io/badge/rust-orange)
 ![gui](https://img.shields.io/badge/gui-orange)

[//]: # (auto_cargo_toml_to_md end)

 ![status](https://img.shields.io/badge/tutorial-yellow)
 ![backup_for_zeljko](https://bestia.dev/webpage_hit_counter/get_svg_image/2117022954.svg)

Hashtags: #tutorial #egui #rust #gui  
My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## GUI for windows

A simple test program that creates a simple GUI for Windows with the crate `egui`.

## Cross compile to windows

On my machine I have Windows11 with WSL/Debian. I will cross compile to Windows, copy the exe file with `scp` and run it on Windows.  
Copy the exe file from the container 'crustde' to win folder. Run in windows git-bash:

```bash
scp rustdevuser@crustde:/home/rustdevuser/rustprojects/backup_for_zeljko/target/x86_64-pc-windows-gnu/release/backup_for_zeljko.exe /c/Users/Luciano/rustprojects/backup_for_zeljko/

# then run in git-bash
cd ~/rustprojects/backup_for_zeljko
./backup_for_zeljko.exe
```

## TODO


## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[//]: # (auto_md_to_doc_comments segment end A)
