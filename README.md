# Dynamic Wallpaper
This program sets the wallpaper of your desktop based on the time of day and your location.
# How to run it
`cargo run -- --day-wallpapers-path <DAY_WALLPAPERS_PATH> --night-wallpapers-path <NIGHT_WALLPAPERS_PATH> --latitude <LATITUDE> --longitude <LONGITUDE>`

Replace `<DAY_WALLPAPERS_PATH>` with the path to the directory containing your daytime wallpapers, `<NIGHT_WALLPAPERS_PATH>` with the path to the directory containing your nighttime wallpapers,`<LATITUDE>` with your latitude, and `<LONGITUDE>` with your longitude.

You can also see a list of available options with the following command:

`cargo run -- --help`


# How to run it in background

1. Build the program in release mode: command `cargo build --release`

2. Link the program to a directory in your $PATH. For example: `ln -s ~/Desktop/dev/dynamic_wallpaper/target/release/dynamic_wallpaper /usr/local/bin`

Replace ~/Desktop/dev/dynamic_wallpaper/target/release/dynamic_wallpaper with the path to your program.

3. Edit the com.xsuto.dynamic_wallpaper.plist file. Change the following lines:


&lt;string&gt;/wallpapers/day&lt;/string&gt;

&lt;string&gt;p/wallpapers/night&lt;/string&gt;

&lt;string&gt;0&lt;/string&gt;

&lt;string&gt;0&lt;/string&gt;

Replace /wallpapers/day with the path to your daytime wallpapers, /wallpapers/night with the path to your nighttime wallpapers, and 0 with your latitude and longitude, respectively.

If you linked the program to a different directory than /usr/local/bin, also update the following line:
&lt;string&gt;/usr/local/bin/dynamic_wallpaper&lt;/string&gt;

4. Link the com.xsuto.dynamic_wallpaper.plist file to the /Library/LaunchDaemons directory: `sudo ln -s ~/Desktop/dev/dynamic_wallpaper/com.xsuto.dynamic_wallpaper.plist /Library/LaunchDaemons`

Replace ~/Desktop/dev/dynamic_wallpaper/com.xsuto.dynamic_wallpaper.plist with the path to your com.xsuto.dynamic_wallpaper.plist file.

5. Load the launch daemon to start the program: `launchctl load /Library/LaunchDaemons/com.xsuto.dynamic_wallpaper.plist`

6. To stop the program, unload the launch daemon: `launchctl unload /Library/LaunchDaemons/com.xsuto.dynamic_wallpaper.plist`
