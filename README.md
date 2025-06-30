<!-- markdownlint-disable MD041 -->
[//]: # (auto_md_to_doc_comments segment start A)

# backup_for_zeljko

[//]: # (auto_cargo_toml_to_md start)

**Simple backup program tailored for my friend Željko. Made with rust and iced.**  
***version: 2025.630.1133 date: 2025-06-30 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/backup_for_zeljko)***

 ![maintained](https://img.shields.io/badge/maintained-green)
 ![ready-for-use](https://img.shields.io/badge/ready_for_use-green)
 ![tutorial](https://img.shields.io/badge/tutorial-orange)
 ![iced](https://img.shields.io/badge/iced-orange)
 ![rust](https://img.shields.io/badge/rust-orange)
 ![gui](https://img.shields.io/badge/gui-orange)

[//]: # (auto_cargo_toml_to_md end)

 ![License](https://img.shields.io/badge/license-MIT-blue.svg)
 ![backup_for_zeljko](https://bestia.dev/webpage_hit_counter/get_svg_image/2117022954.svg)

Hashtags: #tutorial #iced #rust #gui  
My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## Storage

My friend needs to store about 1,5TB of files, mostly photos.
He has 2 laptops with little storage on the internal SSD. We could never use the internal storage to store all of his files. We can use the internal SSD just for "temporary working copies" of some of the files. Eventually we must store the files to a persistent storage.

2TB should be enough to store all his files.  
He bought a 2TB super tiny and extremely fast Samsung T7 Shield external SSD. This will be the main persistent storage of his "original files".  
He has also a 2TB old external HDD as the backup drive.  

## Original location

To avoid any confusion, files and folder must have one and only one "original location".  If anything happens, we know where to find our "original files". The fast external SSD is great for that. This will be the persistent storage of all his "original files".

For some projects it is best to work directly with the "original files". For example when renaming photos and folders.

## Temporary working copy

When working on a project with many small files, first copy the "original files" from the external SSD into the internal "temporary working copy" folder.  

Work for some time on that files.  

When a part of the project is finished, copy the files to their "original location". That can be done manually with great control. Do this often and with confidence. The "original" will survive, all the rest is just temporary.

When the project is finished and all the files are copied into "original location" you do not needed the "temporary working files" any more. Delete these files from the "temporary working copy" to avoid any confusion later. The location of the "originals files" must be always in the same place: the external SSD.

## Backup

In the computer world anything can stop working in a second without any warning signs. I witnessed people loosing all their files because a hard disk died. I saw them cry.

We must always have a backup of the files. It is tedious and it looks superfluous until the disaster strikes.

I will use `robocopy` to make the backup or mirror from the external SSD to the old external HDD.

Never change manually the files on the backup HDD. Only use the robocopy command.

## GUI for windows

My friend is not a computer guy, so I decided that a CLI program in a terminal is not for him. He is comfortable to use GUI programs in Windows.

I will use the crate `iced` to create a simple GUI program for Windows. It is "retained mode GUI".

First the program will check what disks are connected. The names of the folders are fixed so I can recognize them easily. The letters of the external disks can get mounted differently, therefore there is an exploration phase.

1. x:\original_1
2. y:\backup_of_original_1

I will use `robocopy` to make a "mirror backup". Sounds easy.

## Cross compile to windows

On my machine I have Windows11 with WSL/Debian. I will cross compile to Windows, copy the exe file with `scp` and run it on Windows.  

I use `cargo-auto` for automation of the build process and to commit to GitHub. Just run `cargo auto` and follow the instructions. To work with GitHub it will need the Personal Access Token from <https://github.com/settings/tokens>.  

Copy the exe file from the container 'crustde' to win folder. Run in windows git-bash:

```bash
mkdir -p ~/git-bash/rustprojects/backup_for_zeljko
cd ~/git-bash/rustprojects/backup_for_zeljko
scp rustdevuser@crustde:/home/rustdevuser/rustprojects/backup_for_zeljko/target/x86_64-pc-windows-gnu/release/backup_for_zeljko.exe /c/Users/Luciano/git-bash/rustprojects/backup_for_zeljko/

# then run in git-bash
./backup_for_zeljko.exe
```

## Robocopy

Robocopy stands for "Robust copy" in Windows. It is good.  


```bash
robocopy options
/X :: report all eXtra files, not just those selected.
/FP :: include Full Pathname of files in the output.
/NS :: No Size - don't log file sizes.
/NC :: No Class - don't log file classes.
/NDL :: No Directory List - don't log directory names.

robocopy d:\original_1 d:\backup_of_original_1 /MIR /X /FP /NS /NC /NDL

```

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
