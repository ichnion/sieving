# sieving

``sieving`` is a scanner that allows users to hide sensitive data from their files. It is really useful if you want to share your files without spreading precious data, but also if you store for a long time your files, and want to prevent yourself from any leak.

## Installing Sieving
There are different ways to install ``sieving``. The first method is particularly meant to be used by non-developer users.

### *Non-developer users

First click on "Releases" :

You should see the last release on your screen (v0.2.0 when we are writing this).
Depending on your OS (Windows, Linux or Mac), click on one the three files highlighted.
![image](https://user-images.githubusercontent.com/79226149/135598341-c1d8de11-7a0c-4292-a16b-cdd969506141.png)

It should download the file on your device. Then you need to uncompress it.

----------------------------
#### Windows 
We are using 7zip to unzip ``sieving-windows.tar.gz``.
Put the extracted file in `C:\Windows\System32`, and the installation is completed. You should now be able to use ``sieving`` in a terminal.

----------------------------
#### Linux
Extract `sieving-linux.tar.gz` wherever you want on your device.
Finally, open a terminal and type the following command `sudo cp PATH_TO_SIEVING /usr/local/bin` (replace `PATH_TO_SIEVING` by the path to where you extracted ``sieving`` in the previous step). 

The installation is completed, you should now be able to use ``sieving`` in a terminal.

----------------------------
#### MacOS
Extract `sieving-linux.tar.gz` in usr/local/bin.

The installation is completed, you should now be able to use Excavator in a terminal.

----------------------------
### *Developer users

This part is meant to users who want to contribute to the development of ``sieving``. You will need first to have the Rust Programming Language installed on your device. 
Once, you have Rust, you can easily install ``sieving`` from crates.io. 
```sh
$ cargo install sieving
```
----------------------------

## How to use ``sieving``?

``sieving`` takes only one argument : the path to the file (or directory) you want to analyze and protect. 
Open a terminal and type the following command: 
```sh
$ sieving PATH_TO_YOUR_FILE_OR_DIRECTORY
```
Then ``sieving`` will analyze your files and eventually detect some sensitive data.
For example with this file: 
![image](https://user-images.githubusercontent.com/79226149/135600988-509f8fa5-697f-4389-9ad2-c347d8e1a777.png)

if we run ``sieving``:
![image](https://user-images.githubusercontent.com/79226149/135600570-da9b5c26-c64d-4e2b-8a09-3f03602abea8.png)

You need to type ``yes`` or ``no`` depending on if you want to hide data or not. 

For our example, here is the file after scanning it: 

![image](https://user-images.githubusercontent.com/79226149/135601227-0c2c5473-3495-43e7-94f7-1955a175d944.png)

## What sensitive data are currently managed by ``sieving``? 

It is really easy for developers to contribute to ``sieving`` and particularly to add new sensitive data managed by the scanner. So if you have any idea of what data could/should be hidden, please feel free to open an issue. 
Currently, ``sieving`` manages : 

-``Email addresses``

-``IP addresses``

-``IBAN``

