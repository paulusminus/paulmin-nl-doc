+++
date = "2023-11-07"
title = "Lipl Book"
template = "lipl.html"
page_template = "lipl.html"
+++

# Introduction

I found that remembering melodies is easier than words.

When I was organizing sing-alongs with some friends I thought is was useful to display lyrics on a screen instead of working with printed material. So I wrote applications that assisted in storing and updating lyrics and playlists, selecting a lyric to be displayed. Together they are caled Lipl. It is a sort of abbreviation of **L**yr**i**c **Pl**ay.

Lipl consist of three applications

1. An application for storing the lyrics and playlists, called [Lipl Storage](#lipl-storage), accessible through a REST interface.
2. An application for displaying part of a lyric on a screen, called [Lipl Display](#lipl-display).
3. An application for maintaining the collection of lyrics and playlists, and selecting a lyric or playlist that should be displayed on a screen, called [Lipl Control](#lipl-control).

The first component is written in rust. It runs on a private virtual linux server accessible through the internet. It has a restful interface for creating, reading, updating and deleting of lyrics and playlists.

The second component is also written in rust. The application runs on a Raspberry Pi raspbian installed.
After automatic login of a user, the software starts a gatt peripheral. Bluez has a dbus interface that is used to define the gatt peripheral.

The third component is written in Dart and uses Flutter. It runs on an Android phone. It also provides a web interface used to maintain the lyrics and playlists used.

## Sing-along

The sing-alongs are not at home. When I go to a sing-along I bring with me
- my smartphone with lipl-control installed and data synchronized with lipl-storage
- a raspberry Pi with lipl-display installed

The Raspberry Pi gets connected to a display with the help of its HDMI connector. After booting the Pi advertises a Gatt Service.

Lipl-control can listen for advertisements and connect to the Pi. When I choose Play on lipl-control the parts and status are written to Gatt Characteristics on lipl-display. Lipl control can also write to a characteric that controls font size and theme.


# Lipl Storage

Application that starts a webserver accepting requests to create, read, update of delete lyrics or playlists.
You can use multiple ways of storing data, e.g. file, postgres, redis and memory. The last one is non persistent and is only useful in test scenario's.
Handling of authentication, encryption or compression can be done by a reverse proxy server, e.g. apache.

Request handling is is done by [axum](https://crates.io/crates/axum).

[The Source code of Lipl Storage](https://www.github.com/paulusminus/lipl-storage) can be found on Github.


# Lipl Display

The application is using the primary UI thread for the UI. A second thread is started that starts a Gatt peripheral that handles characteristic writes, converts them to messages and passes them to the UI thread.

The gatt peripheral part defines a Gatt service with three characeristics that are writeable, namely text, status and control.

Text is used to get the part of the lyrics that needs to be shown.

Status is used to get the text for the statusbar that is shown on the bottom of the screen, typically the title, the current part and the total parts of het lyric being displayed.

Control is used to select a theme, black letters on a white screen or white letters on a black screen, or to increase or decrease the fontsize.

Source code for the [flutter] and [slint] version is available on Github. I prefer using the slint version because you can run it on raspbian lite.

[flutter]: https://www.github.com/paulusminus/lipl-display-flutter
[slint]: https://www.github.com/paulusminus/lipl-display


# Lipl Control

If the device the application is running on has an Internet Connection, the application can synchronize its data with Lipl Storage. Synchronization is important so the application can also be used when there is no Internet Connection.

On Android the application can establish a connection with Lipl Display. Lipl Display uses advertising so the device can be found.
If the connection is established, you can use the play button to display parts of a lyric or playlist.

Flutter can also build a web app. This can be handy for people maintaining the data. Bluetooth connection are not possible though.

[Source code for Lipl Control](https://www.github.com/paulusminus/lipl-control-flutter) can be found on Github.
