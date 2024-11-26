[//]: # (auto_md_to_doc_comments segment start A)

# backup_for_zeljko

[//]: # (auto_cargo_toml_to_md start)

**counter GUI with egui**  
***version: 2024.1127.1952 date: 2024-11-27 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/backup_for_zeljko)***

 ![tutorial](https://img.shields.io/badge/tutorial-orange)
 ![egui](https://img.shields.io/badge/egui-orange)
 ![rust](https://img.shields.io/badge/rust-orange)
 ![gui](https://img.shields.io/badge/gui-orange)

[//]: # (auto_cargo_toml_to_md end)

 ![status](https://img.shields.io/badge/tutorial-yellow)
 ![backup_for_zeljko](https://bestia.dev/webpage_hit_counter/get_svg_image/2117022954.svg)

Hashtags: #tutorial #egui #rust #gui  
My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## Backup

My friend has a laptop with 1TB space for his data.  
Then a super tiny and extremely fast Samsung T7 Shield 2TB external SSD as primary backup.  
And then a big old 2TB external HDD as secondary backup. It is so big that it has it's own power.  

The difference in storage size makes the backup tricky. Not impossible, but a notch more complicated that he would like.

I was thinking of using `robocopy` for the backup, but as always it lacks a feature that I would like to have.
When the files are deleted on the `original` disk I want to save these files on the backup disk for possible recovery later. Robocopy just delete them. I know there exists the windows recycle bin, but I don't want to use that.  

First the program will check what disks are connected. The names of the folders are fixed so I can recognize them easily. There are 3 different backups:

1. from the laptop d:\aktivne_datoteke to e:\backup_1\aktivne_datoteke
2. from the laptop d:\aktivne_datoteke to f:\backup_2\aktivne_datoteke
3. from the primary backup e:\backup_1\arhivirane_datoteke to f:\backup_2\arhivirane_datoteke

The files that should be deleted will be moved into a folder named `zbrisane_datoteke_datetime`. This can be reviewed and finally manually delete when not needed any more.

I will use `robocopy` to list the files to be deleted. Then I will parse the output and move these files.
Then I will use `robocopy` to make a "mirror backup". Sounds easy.

## GUI for windows

My friend is not a computer guy, so I decided that a CLI program in a terminal is not for him. He is comfortable to use GUI programs in Windows.

I will experiment with the crate `egui` to create a super simple GUI program for Windows.

## Cross compile to windows

On my machine I have Windows11 with WSL/Debian. I will cross compile to Windows, copy the exe file with `scp` and run it on Windows.  
Copy the exe file from the container 'crustde' to win folder. Run in windows git-bash:

```bash
scp rustdevuser@crustde:/home/rustdevuser/rustprojects/backup_for_zeljko/target/x86_64-pc-windows-gnu/release/backup_for_zeljko.exe /c/Users/Luciano/rustprojects/backup_for_zeljko/

# then run in git-bash
cd ~/rustprojects/backup_for_zeljko
./backup_for_zeljko.exe
```

## Robocopy

Robocopy stands for "Robust copy" in Windows. It is good.  
First I want to list the files that should be deleted.  
Then I will parse the output and move these files instead of delete them.  
In the output I must recognize the files that start with the `destination` folder.

```bash
robocopy options
/L :: List only - don't copy, timestamp or delete any files.
/X :: report all eXtra files, not just those selected.
/FP :: include Full Pathname of files in the output.
/NS :: No Size - don't log file sizes.
/NC :: No Class - don't log file classes.
/NDL :: No Directory List - don't log directory names.

robocopy d:\aktivne_datoteke d:\backup_1\aktivne_datoteke /MIR /L /X /FP /NS /NC /NDL

The output is terrible to parse:
-------------------------------------------------------------------------------
   ROBOCOPY     ::     Robust File Copy for Windows
-------------------------------------------------------------------------------

  Started : torek, 26. november 2024 11:03:11
   Source : d:\aktivne_datoteke\
     Dest : d:\backup_1\aktivne_datoteke\

    Files : *.*

  Options : *.* /X /FP /NS /NC /NDL /L /S /E /DCOPY:DA /COPY:DAT /PURGE /MIR /R:1000000 /W:30

------------------------------------------------------------------------------

                                d:\backup_1\aktivne_datoteke\LF2023-12-15 10-46-45 minja es alta.jpg
                                d:\aktivne_datoteke\LF2023-12-11 11-07-09 Minja Luciano.jpg

------------------------------------------------------------------------------

               Total    Copied   Skipped  Mismatch    FAILED    Extras
    Dirs :         2         0         2         0         0         0
   Files :         6         1         5         0         0         2
   Bytes :   31.83 m    9.06 m   22.77 m         0         0    2.70 m
   Times :   0:00:00   0:00:00                       0:00:00   0:00:00
   Ended : torek, 26. november 2024 11:03:11
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
