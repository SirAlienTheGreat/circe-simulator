# circe-simulator
<h1>Run instructions</h1>
<h1>This was a class project from 2022 that is not useful to anyone and archived for that reason</h1>
<ol>
  <li>Download the release from your platform on the releases page on the right side of this page</li>
  <li>Open the circe-simulator-*****.zip file and copy the contents to an empty folder somewhere</li>
  <li>DO NOT move any of the included files into a seperate folder</li>
  <li>Double click the file that says "circe-simulator" or "circe-simulator.exe"
</ol>

<h1> Linux build instructions </h1>
<p> If you don't know what this is, don't do this</p>
<ol>
  <li>Most distros come with gtk4 preinstalled, if yours doesn't, use <samp>sudo pacman -S gtk4</samp> or <samp>sudo apt install libgtk-4-dev</samp> (Probably, I don't use debian)</li>
  <li>Install rust with <samp>curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh</samp> or use your distro's package manager</li>
  <li>Clone this repository with <samp>git clone https://github.com/SirAlienTheGreat/circe-simulator</samp>, then <samp>cd</samp> into this directory</li>
  <li>Build with <samp>cargo build --release</samp></li>
  <li>Copy the image files into <samp>target/release</samp></li>
  <li>You may need to <samp>chmod +x circe-simulator</samp>, or it may have permission to run with <samp>./circe-simulator</samp> by default</li>
</ol>

<h1> Windows build instructions (Not recommended) (Untested) </h1>

<ol>
  <li>Download and install MSYS2 at https://www.msys2.org/</li>
  <li>Download and install rust with the .exe file at https://www.rust-lang.org/ but when it asks if you want a custom installation,
  answer yes, and change the toolkit to x86_64-pc-windows-gnu</li>
  <li>Open the MinGW shell. This is a Linux-like build environment for windows.</li>
  <li>Install dependencies by running <samp>pacman -S mingw-w64-x86_64-gtk4</samp> and <samp>pacman -S mingw-w64-x86_64-toolchain</samp></li>
  <li>Clone this repository with <samp>git clone https://github.com/SirAlienTheGreat/circe-simulator</samp>, then <samp>cd</samp> into this directory</li>
  <li>Build with <samp>cargo build --release</samp></li>
  <li>Pray to your favorite god, because this never works for me, and this sometimes seems to fix itself randomly</li>
  <li>Copy the image files into <samp>target/release</samp></li>
  <li><samp>cd</samp> into <samp>target/release</samp> and add the .dlls with <samp>ldd circe-simulator.exe | grep '\/mingw64\/bin\/.*dll' -o | xargs -I{} cp "{}" .</samp> (the period is part of the command)</li>
  <li>You can open file explorer and go to your MSYS2 installation folder (By default, C:msys2)<samp>/home/[USERNAME]/circe-simulator-main/target/release</samp> and run circe-simulator.exe</li>
</ol>

<samp></samp>
